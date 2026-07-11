#!/usr/bin/env python3

from __future__ import annotations

import argparse
import fnmatch
import re
import shutil
import subprocess
import sys
import tomllib
from collections import deque
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONFIG_PATH = Path("scripts/check.toml")


def load_config(root: Path) -> dict:
    with (root / CONFIG_PATH).open("rb") as file:
        return tomllib.load(file)


def relative_paths(section: dict, key: str) -> list[Path]:
    return [Path(path) for path in section.get(key, [])]


def required(section: dict, key: str) -> str:
    value = section.get(key)
    if not isinstance(value, str) or not value:
        raise ValueError(f"missing required config value: {key}")
    return value


class RuntimeCheck:
    DLL_RE = re.compile(r"DLL Name: (.+)")

    class Profile:
        def __init__(self, name: str, arch: Path, entries: list[str]) -> None:
            self.name = name
            self.arch = arch
            self.entries = entries

    class ArchResult:
        def __init__(self, arch_dir: Path, profile_name: str) -> None:
            self.arch_dir = arch_dir
            self.profile_name = profile_name
            self.visited: list[str] = []
            self.local_edges: list[tuple[str, str]] = []
            self.system_edges: list[tuple[str, str]] = []
            self.missing: list[tuple[str, str]] = []

    def __init__(
        self,
        root: Path,
        profiles: list[RuntimeCheck.Profile],
        system_dlls: set[str],
        objdump: str,
    ) -> None:
        self.root = root
        self.profiles = profiles
        self.system_dlls = system_dlls
        self.objdump = objdump

    @classmethod
    def from_config(
        cls,
        root: Path,
        config: dict,
        objdump: str,
        arches: list[Path] | None = None,
        entries: list[str] | None = None,
    ) -> RuntimeCheck:
        system_dlls, profiles = cls.load_config(config)
        default_entries = profiles[0].entries if profiles else []

        if arches:
            by_arch = {profile.arch: profile for profile in profiles}
            selected_profiles = []
            for arch in arches:
                profile = by_arch.get(arch)
                selected_profiles.append(
                    cls.Profile(
                        name=profile.name if profile else str(arch),
                        arch=arch,
                        entries=entries or (profile.entries if profile else default_entries),
                    )
                )
            profiles = selected_profiles
        elif entries:
            profiles = [
                cls.Profile(profile.name, profile.arch, entries) for profile in profiles
            ]

        return cls(root=root, profiles=profiles, system_dlls=system_dlls, objdump=objdump)

    @classmethod
    def load_config(cls, config: dict) -> tuple[set[str], list[RuntimeCheck.Profile]]:
        system_dlls = {
            dll.upper() for dll in config.get("win", {}).get("system_libs", [])
        }
        profiles = [
            cls.Profile(
                name=profile["name"],
                arch=Path(profile["arch_dir"]),
                entries=list(profile["entries"]),
            )
            for profile in config.get("runtime_profiles", [])
        ]
        return system_dlls, profiles

    @staticmethod
    def find_objdump(
        root: Path | None = None,
        names: list[str] | None = None,
        fallback_paths: list[Path] | None = None,
    ) -> str | None:
        for name in names or []:
            path = shutil.which(name)
            if path:
                return path

        if root is not None and fallback_paths is not None:
            for path in fallback_paths:
                resolved = root / path
                if resolved.is_file():
                    return str(resolved)

        return None

    def run(self) -> list[RuntimeCheck.ArchResult]:
        return [self.check_arch(profile) for profile in self.profiles]

    def check_arch(self, profile: RuntimeCheck.Profile) -> RuntimeCheck.ArchResult:
        arch_dir = self.root / profile.arch
        result = self.ArchResult(arch_dir, profile.name)
        if not arch_dir.is_dir():
            result.missing.append((str(arch_dir), "<arch>"))
            return result

        local_files = {
            path.name.lower(): path.name for path in arch_dir.iterdir() if path.is_file()
        }
        queue = deque(profile.entries)
        visited_keys: set[str] = set()

        while queue:
            dll = queue.popleft()
            key = dll.lower()
            if key in visited_keys:
                continue
            visited_keys.add(key)
            result.visited.append(local_files.get(key, dll))

            dll_path = arch_dir / local_files.get(key, dll)
            if not dll_path.exists():
                result.missing.append((dll, "<entry>"))
                continue

            for dependency in self.imported_dlls(dll_path):
                dependency_key = dependency.lower()
                if dependency.upper() in self.system_dlls:
                    result.system_edges.append((dll_path.name, dependency))
                elif dependency_key in local_files:
                    local_dependency = local_files[dependency_key]
                    result.local_edges.append((dll_path.name, local_dependency))
                    if dependency_key not in visited_keys:
                        queue.append(local_dependency)
                else:
                    result.missing.append((dependency, dll_path.name))

        return result

    def imported_dlls(self, dll_path: Path) -> list[str]:
        completed = subprocess.run(
            [self.objdump, "-p", str(dll_path)],
            check=True,
            capture_output=True,
            text=True,
        )
        return self.DLL_RE.findall(completed.stdout)

    def print_text(self, results: list[RuntimeCheck.ArchResult]) -> None:
        print("runtime DLL dependency check")
        for result in results:
            status = "ok" if not result.missing else "missing"
            print(
                f"  {status}: {result.profile_name} "
                f"({result.arch_dir}, {len(result.visited)} DLLs checked)"
            )

            for dll, source in result.missing:
                print(f"    {dll} imported by {source}")


def check_env(args: argparse.Namespace) -> int:
    root = args.root.resolve()
    config = load_config(root)
    env_config = config.get("env", {})
    objdump_names = list(env_config.get("objdump_names", []))
    fallback_paths = relative_paths(env_config, "objdump_fallbacks")
    objdump = args.objdump or RuntimeCheck.find_objdump(
        root,
        names=objdump_names,
        fallback_paths=fallback_paths,
    )
    if not objdump:
        print("error: llvm-objdump not found on PATH", file=sys.stderr)
        return 2

    check = RuntimeCheck.from_config(
        root=root,
        config=config,
        arches=args.arch,
        entries=args.entry,
        objdump=objdump,
    )
    results = check.run()
    check.print_text(results)

    return 1 if any(result.missing for result in results) else 0


def check_commit_msg(args: argparse.Namespace) -> int:
    config = load_config(ROOT)
    commit_config = config.get("commit", {})
    pattern = re.compile(required(commit_config, "pattern"))
    lines = args.message.read_text(encoding="utf-8-sig").splitlines()
    first_line = lines[0].strip() if lines else ""
    if pattern.fullmatch(first_line):
        return 0

    print("commit message must use 'type: message' format", file=sys.stderr)
    print("optional scope and breaking marker are supported", file=sys.stderr)
    return 1


def check_docs(args: argparse.Namespace) -> int:
    root = args.root.resolve()
    config = load_config(root).get("docs", {})
    docs_dir = root / required(config, "docs_dir")
    pages_file = root / required(config, "site_pages")
    reference_re = re.compile(config["reference_pattern"])
    ignore_unreferenced = list(config.get("ignore_unreferenced", []))

    missing: list[Path] = []
    if not docs_dir.is_dir():
        missing.append(docs_dir)
    if not pages_file.is_file():
        missing.append(pages_file)
    if missing:
        print("docs check failed: missing required paths", file=sys.stderr)
        for path in missing:
            print(f"  {path}", file=sys.stderr)
        return 1

    pages_source = pages_file.read_text(encoding="utf-8")
    referenced = sorted(set(reference_re.findall(pages_source)))
    missing_docs = [path for path in referenced if not (docs_dir / path).is_file()]

    if missing_docs:
        print("docs check failed: missing referenced docs", file=sys.stderr)
        for path in missing_docs:
            print(f"  docs/{path}", file=sys.stderr)
        return 1

    docs_files = {
        path.relative_to(docs_dir).as_posix() for path in docs_dir.rglob("*.md")
    }
    intentionally_unlisted = {
        path for path in docs_files if ignored(path, ignore_unreferenced)
    }
    unreferenced = sorted(docs_files - set(referenced) - intentionally_unlisted)

    print(f"docs check ok: {len(referenced)} site doc references")
    if unreferenced:
        print("  unreferenced docs:")
        for path in unreferenced:
            print(f"    docs/{path}")
    return 0


def ignored(path: str, patterns: list[str]) -> bool:
    return any(fnmatch.fnmatch(path, pattern) for pattern in patterns)


def read_recipe_source(recipe: Path) -> Path:
    in_source = False
    for line in recipe.read_text(encoding="utf-8").splitlines():
        if line.startswith("source:"):
            in_source = True
            continue
        if in_source and line and not line.startswith((" ", "\t")):
            break
        if in_source:
            match = re.match(r"\s*path:\s*(.+)\s*$", line)
            if match:
                return Path(match.group(1).strip().strip("'\""))
    raise ValueError(f"source.path not found in {recipe}")


def check_package(args: argparse.Namespace) -> int:
    root = args.root.resolve()
    config = load_config(root).get("package", {})
    package_root = root / required(config, "root")
    recipe = package_root / required(config, "recipe")

    missing: list[Path] = []
    if not recipe.is_file():
        missing.append(recipe)
        source_root = package_root / "sources"
    else:
        try:
            source_root = package_root / read_recipe_source(recipe)
        except ValueError as error:
            print(f"package check failed: {error}", file=sys.stderr)
            return 1

    required_paths = [recipe]
    required_paths.extend(
        source_root / path for path in relative_paths(config, "required_source_files")
    )

    missing.extend(path for path in required_paths if not path.exists())
    if missing:
        print("package check failed: missing required files", file=sys.stderr)
        for path in missing:
            print(f"  {path}", file=sys.stderr)
        return 1

    print(f"package check ok: {source_root.relative_to(root).as_posix()}")
    return 0


def add_env_parser(subparsers: argparse._SubParsersAction) -> None:
    parser = subparsers.add_parser(
        "env",
        help="validate vendored runtime DLL dependencies",
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=ROOT,
        help="workspace root; defaults to this script's workspace",
    )
    parser.add_argument(
        "--arch",
        action="append",
        type=Path,
        help="runtime DLL directory to check; may be passed multiple times",
    )
    parser.add_argument(
        "--entry",
        action="append",
        help="entry DLL to start from; may be passed multiple times",
    )
    parser.add_argument(
        "--objdump",
        help="llvm-objdump executable; defaults to PATH lookup",
    )
    parser.set_defaults(func=check_env)


def main() -> int:
    parser = argparse.ArgumentParser(description="Run hikcamera-rs repository checks.")
    subparsers = parser.add_subparsers(dest="command")

    add_env_parser(subparsers)
    runtime_parser = subparsers.add_parser(
        "runtime",
        help="alias for env",
    )
    runtime_parser.set_defaults(func=check_env, root=ROOT, arch=None, entry=None, objdump=None)

    commit_parser = subparsers.add_parser(
        "commit-msg",
        help="validate the first line of a git commit message",
    )
    commit_parser.add_argument("message", type=Path)
    commit_parser.set_defaults(func=check_commit_msg)

    docs_parser = subparsers.add_parser(
        "docs",
        help="validate docs files referenced by the site",
    )
    docs_parser.add_argument("--root", type=Path, default=ROOT)
    docs_parser.set_defaults(func=check_docs)

    package_parser = subparsers.add_parser(
        "package",
        help="validate vendored conda package inputs",
    )
    package_parser.add_argument("--root", type=Path, default=ROOT)
    package_parser.set_defaults(func=check_package)

    argv = sys.argv[1:] or ["env"]
    args = parser.parse_args(argv)
    if not hasattr(args, "func"):
        parser.print_help()
        return 2
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
