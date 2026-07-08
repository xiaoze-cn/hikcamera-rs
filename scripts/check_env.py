#!/usr/bin/env python3

from __future__ import annotations

import argparse
import re
import sys
import shutil
import subprocess
import tomllib
from collections import deque
from pathlib import Path


class RuntimeCheck:
    DEFAULT_CONFIG = Path("scripts/env/win_dll.toml")
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
        objdump: str,
        arches: list[Path] | None = None,
        entries: list[str] | None = None,
    ) -> RuntimeCheck:
        system_dlls, profiles = cls.load_config(root / cls.DEFAULT_CONFIG)
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
    def load_config(cls, config_path: Path) -> tuple[set[str], list[RuntimeCheck.Profile]]:
        with config_path.open("rb") as file:
            config = tomllib.load(file)

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
    def find_objdump(root: Path | None = None) -> str | None:
        for name in ("llvm-objdump", "llvm-objdump.exe"):
            path = shutil.which(name)
            if path:
                return path

        if root is not None:
            for path in (
                root / ".pixi/envs/default/Library/bin/llvm-objdump.exe",
                root / ".pixi/envs/default/bin/llvm-objdump",
            ):
                if path.is_file():
                    return str(path)

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

def main() -> int:
    parser = argparse.ArgumentParser(description="Check HikCamera environment.")
    subparsers = parser.add_subparsers(dest="command")
    runtime_parser = subparsers.add_parser(
        "runtime",
        help="validate vendored runtime DLL dependencies",
    )
    runtime_parser.add_argument(
        "--root",
        type=Path,
        default=Path.cwd(),
        help="workspace root; defaults to the current directory",
    )
    runtime_parser.add_argument(
        "--arch",
        action="append",
        type=Path,
        help="runtime DLL directory to check; may be passed multiple times",
    )
    runtime_parser.add_argument(
        "--entry",
        action="append",
        help="entry DLL to start from; may be passed multiple times",
    )
    runtime_parser.add_argument(
        "--objdump",
        help="llvm-objdump executable; defaults to PATH lookup",
    )

    argv = sys.argv[1:]
    if not argv:
        argv = ["runtime"]

    args = parser.parse_args(argv)
    if args.command != "runtime":
        parser.print_help()
        return 2

    root = args.root.resolve()
    objdump = args.objdump or RuntimeCheck.find_objdump(root)
    if not objdump:
        print("error: llvm-objdump not found on PATH", file=sys.stderr)
        return 2

    check = RuntimeCheck.from_config(
        root=root,
        arches=args.arch,
        entries=args.entry,
        objdump=objdump,
    )
    results = check.run()
    check.print_text(results)

    return 1 if any(result.missing for result in results) else 0


if __name__ == "__main__":
    raise SystemExit(main())
