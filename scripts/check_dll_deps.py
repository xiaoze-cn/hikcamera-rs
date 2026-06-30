#!/usr/bin/env python3
"""Validate vendored HikCamera runtime DLL dependencies.

The script walks PE import tables from selected entry DLLs and verifies that
all non-system DLL imports are present in the same architecture directory.
"""

from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
from collections import defaultdict, deque
from pathlib import Path

DEFAULT_ARCH_DIRS = (
    Path("crates/hikcamera-sys/lib/win32"),
    Path("crates/hikcamera-sys/lib/win64"),
)
DEFAULT_ENTRIES = ("MvCameraControl.dll", "MvISPControl.dll")
DLL_RE = re.compile(r"DLL Name: (.+)")

# Windows DLLs that are expected to be provided by the OS rather than vendored
# next to the HikCamera runtime.
SYSTEM_DLLS = {
    "ADVAPI32.DLL",
    "COMCTL32.DLL",
    "COMDLG32.DLL",
    "DBGHELP.DLL",
    "GDI32.DLL",
    "KERNEL32.DLL",
    "OLE32.DLL",
    "OLEAUT32.DLL",
    "RPCRT4.DLL",
    "SHELL32.DLL",
    "USER32.DLL",
    "VERSION.DLL",
    "WINMM.DLL",
    "WS2_32.DLL",
}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root",
        type=Path,
        default=Path.cwd(),
        help="workspace root; defaults to the current directory",
    )
    parser.add_argument(
        "--arch-dir",
        action="append",
        type=Path,
        help="architecture runtime directory to check; may be passed multiple times",
    )
    parser.add_argument(
        "--entry",
        action="append",
        default=list(DEFAULT_ENTRIES),
        help="entry DLL to start from; may be passed multiple times",
    )
    parser.add_argument(
        "--objdump",
        default=find_objdump(),
        help="llvm-objdump executable; defaults to PATH lookup",
    )
    parser.add_argument(
        "--format",
        choices=("text", "markdown"),
        default="text",
        help="output format",
    )
    args = parser.parse_args()

    if not args.objdump:
        print("error: llvm-objdump not found on PATH", file=sys.stderr)
        return 2

    root = args.root.resolve()
    arch_dirs = args.arch_dir or list(DEFAULT_ARCH_DIRS)
    results = [
        check_arch(root / arch_dir, args.entry, args.objdump) for arch_dir in arch_dirs
    ]

    if args.format == "markdown":
        print_markdown(results)
    else:
        print_text(results)

    return 1 if any(result.missing for result in results) else 0


def find_objdump() -> str | None:
    for name in ("llvm-objdump", "llvm-objdump.exe"):
        path = shutil.which(name)
        if path:
            return path
    return None


class ArchResult:
    def __init__(self, arch_dir: Path) -> None:
        self.arch_dir = arch_dir
        self.visited: list[str] = []
        self.local_edges: list[tuple[str, str]] = []
        self.system_edges: list[tuple[str, str]] = []
        self.missing: list[tuple[str, str]] = []


def check_arch(arch_dir: Path, entries: list[str], objdump: str) -> ArchResult:
    result = ArchResult(arch_dir)
    local_files = {
        path.name.lower(): path.name for path in arch_dir.iterdir() if path.is_file()
    }
    queue = deque(entries)
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

        for dependency in imported_dlls(objdump, dll_path):
            dependency_key = dependency.lower()
            if dependency.upper() in SYSTEM_DLLS:
                result.system_edges.append((dll_path.name, dependency))
            elif dependency_key in local_files:
                local_dependency = local_files[dependency_key]
                result.local_edges.append((dll_path.name, local_dependency))
                if dependency_key not in visited_keys:
                    queue.append(local_dependency)
            else:
                result.missing.append((dependency, dll_path.name))

    return result


def imported_dlls(objdump: str, dll_path: Path) -> list[str]:
    completed = subprocess.run(
        [objdump, "-p", str(dll_path)],
        check=True,
        capture_output=True,
        text=True,
    )
    return DLL_RE.findall(completed.stdout)


def print_text(results: list[ArchResult]) -> None:
    for result in results:
        print(f"[{result.arch_dir}]")
        print("local closure:")
        for dll in result.visited:
            print(f"  {dll}")
        print("local imports:")
        for source, target in sorted(set(result.local_edges)):
            print(f"  {source} -> {target}")
        print("system imports:")
        for source, targets in grouped_system_edges(result.system_edges).items():
            print(f"  {source} -> {', '.join(targets)}")
        if result.missing:
            print("missing non-system imports:")
            for dll, source in result.missing:
                print(f"  {dll} imported by {source}")
        else:
            print("missing non-system imports: none")
        print()


def print_markdown(results: list[ArchResult]) -> None:
    for result in results:
        print(f"## `{result.arch_dir}`")
        print()
        print("### Local import edges")
        print()
        print("| From | To |")
        print("| --- | --- |")
        for source, target in sorted(set(result.local_edges)):
            print(f"| `{source}` | `{target}` |")
        print()
        print("### System import groups")
        print()
        print("| From | System DLLs |")
        print("| --- | --- |")
        for source, targets in grouped_system_edges(result.system_edges).items():
            joined = ", ".join(f"`{target}`" for target in targets)
            print(f"| `{source}` | {joined} |")
        print()
        if result.missing:
            print("### Missing non-system imports")
            print()
            print("| Missing DLL | Imported by |")
            print("| --- | --- |")
            for dll, source in result.missing:
                print(f"| `{dll}` | `{source}` |")
            print()


def grouped_system_edges(edges: list[tuple[str, str]]) -> dict[str, list[str]]:
    grouped: dict[str, set[str]] = defaultdict(set)
    for source, target in edges:
        grouped[source].add(target)
    return {source: sorted(targets) for source, targets in sorted(grouped.items())}


if __name__ == "__main__":
    raise SystemExit(main())
