#!/usr/bin/env python3
"""doctor.py -- repo health checker for the rustlearning workspace.

Runs one or more sanity checks on this repository:

  --binary     Find stray compiled artifacts (e.g. ELF executables produced
               by running `rustc main.rs` directly inside src/). Offers one
               collective Y/N prompt to delete all of them.
  --workspace  Verify that every sub-directory containing a Cargo.toml is
               listed in the root workspace members (and vice versa).
               Warning only: never prompts and never deletes anything.
  --git        Find nested `.git` entries (e.g. left behind by a `cargo new`
               without `--vcs none`). Offers one collective Y/N prompt to
               delete all of them.

With no selector flags, all three checks run in order: binary, workspace, git.

--check-only makes every check non-interactive: findings are listed but no
deletion prompt is shown and nothing is removed. Combined with the selector
flags this is convenient for a quick read-only review.

Exit status:
  0  no stray binaries and no nested git repos remain after the run
     (workspace mismatches are warnings and do not affect the exit status)
  1  stray binaries or nested git repos remain (not found, or not deleted)
  2  unexpected error
"""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path

# The repository root is the directory containing this script.
REPO_ROOT = Path(__file__).resolve().parent

# Directories whose contents are legitimate build output or repo metadata;
# they are never scanned and never reported by the binary check. `target/`
# is already covered by .gitignore, and `.git/` belongs to the repo itself.
SKIP_DIRS = {".git", "target"}

# Extensions that unambiguously denote compiled objects even if the file
# magic check below cannot run or misses (e.g. empty files).
BINARY_EXTENSIONS = {".o", ".obj", ".exe", ".dll", ".rlib"}

# File-magic signatures for common executable/object formats. The ELF entry
# is what `rustc` produces on Linux, which is the main thing we hunt for:
# extension-less executables like `src/main` that `git` does not ignore.
# The `ar` entry catches static archives such as `libfoo.rlib` (also from
# a direct `rustc` invocation).
MAGIC_SIGNATURES = (
    (b"\x7fELF", "ELF executable/object"),
    (b"\xcf\xfa\xed\xfe", "Mach-O 64-bit"),
    (b"\xce\xfa\xed\xfe", "Mach-O 32-bit"),
    (b"\xca\xfe\xba\xbe", "Mach-O universal"),
    (b"MZ", "PE/DOS executable"),
    (b"!<arch>\n", "ar archive (e.g. .rlib/.a)"),
)


# --------------------------------------------------------------------------
# Small shared helpers
# --------------------------------------------------------------------------

def ask_yes_no(prompt: str) -> bool:
    """Ask a yes/no question on stdin. Default (empty/unknown/EOF) is No."""
    try:
        answer = input(f"{prompt} [y/N]: ").strip().lower()
    except EOFError:
        # No interactive stdin available (piped/CI context): be safe, say no.
        return False
    return answer in ("y", "yes")


def git_tracked_files() -> set[str]:
    """Return the set of git-tracked paths (posix, repo-relative).

    Used as a safety net: anything already tracked by git is deliberately
    versioned and must never be offered for deletion. Returns an empty set
    if git is unavailable so the check degrades to magic-based detection.
    """
    try:
        proc = subprocess.run(
            ["git", "ls-files", "-z"],
            cwd=REPO_ROOT,
            capture_output=True,
            check=True,
        )
    except (OSError, subprocess.CalledProcessError):
        print("[i] could not query git for tracked files; skipping that filter")
        return set()
    return {entry for entry in proc.stdout.decode("utf-8", "replace").split("\0") if entry}


def describe(path: Path) -> str:
    """One-line human description of a path, used in listings."""
    if path.is_dir():
        return "directory"
    try:
        size = path.stat().st_size
    except OSError:
        size = -1
    return f"file, {size} bytes"


def delete(path: Path) -> None:
    """Remove a file, symlink, or a whole directory tree."""
    if path.is_dir() and not path.is_symlink():
        shutil.rmtree(path)
    else:
        path.unlink()


def offer_deletion(items: list[Path], kind: str, check_only: bool) -> list[Path]:
    """List all findings collectively, then offer one collective Y/N delete.

    Returns the list of items still on disk after the user decision, so the
    caller can count them towards the final exit status.
    """
    print(f"\n[!] Found {len(items)} stray {kind}:")
    for item in sorted(items):
        try:
            shown = item.relative_to(REPO_ROOT)
        except ValueError:
            shown = item
        print(f"    {shown}  ({describe(item)})")

    if check_only:
        print("[i] --check-only given: listing only, nothing is deleted.")
        return items

    if not ask_yes_no(f"Delete ALL {len(items)} {kind} listed above"):
        print("[i] Skipped, nothing deleted.")
        return items

    survivors: list[Path] = []
    for item in sorted(items):
        try:
            delete(item)
            print(f"    deleted {item.relative_to(REPO_ROOT)}")
        except OSError as exc:
            print(f"    [x] failed to delete {item.relative_to(REPO_ROOT)}: {exc}")
            survivors.append(item)
    return survivors


def is_probably_binary(path: Path) -> str | None:
    """Return a format description if the file looks like compiled output."""
    if path.suffix.lower() in BINARY_EXTENSIONS:
        return f"{path.suffix} object/executable"
    try:
        with path.open("rb") as handle:
            head = handle.read(4)
    except OSError:
        return None
    for magic, label in MAGIC_SIGNATURES:
        if head.startswith(magic):
            return label
    return None


def find_stray_binaries() -> list[Path]:
    """Walk the repo (minus SKIP_DIRS) and collect compiled artifacts.

    Files tracked by git are excluded: being tracked means they were put
    there on purpose, however odd that would be.
    """
    tracked = git_tracked_files()
    findings: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(REPO_ROOT):
        # Prune in-place so os.walk does not descend into these dirs.
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for name in filenames:
            path = Path(dirpath) / name
            rel = path.relative_to(REPO_ROOT).as_posix()
            if rel in tracked:
                continue
            label = is_probably_binary(path)
            if label is not None:
                findings.append(path)
    return sorted(findings)


def check_binaries(check_only: bool) -> list[Path]:
    """Entry point for the --binary check."""
    print("== check: stray compiled artifacts ==")
    findings = find_stray_binaries()
    if not findings:
        print("OK, no stray binaries outside target/.")
        return []
    return offer_deletion(findings, "compiled artifact(s)", check_only)


def expand_members(members: list[str]) -> set[str]:
    """Resolve workspace member entries to a set of directory names.

    Supports glob-style entries such as `ch*` the same way cargo does.
    """
    resolved: set[str] = set()
    for entry in members:
        normalized = entry.strip().strip("./").rstrip("/")
        if not normalized:
            continue
        if any(char in normalized for char in "*?["):
            for path in REPO_ROOT.glob(normalized):
                if path.is_dir():
                    resolved.add(path.name)
        else:
            resolved.add(normalized)
    return resolved


def find_crate_dirs() -> set[str]:
    """Sub-directories of the repo root that contain a Cargo.toml.

    Directories without a Cargo.toml (docs, notes, ...) are not crates and
    are deliberately ignored by this check.
    """
    return {
        entry.name
        for entry in REPO_ROOT.iterdir()
        if entry.is_dir()
        and not entry.name.startswith(".")
        and (entry / "Cargo.toml").is_file()
    }


def check_workspace() -> list[str]:
    """Entry point for the --workspace check. Warning-only, never deletes."""
    print("== check: workspace members vs directories ==")
    warnings: list[str] = []

    root_manifest = REPO_ROOT / "Cargo.toml"
    try:
        with root_manifest.open("rb") as handle:
            manifest = tomllib.load(handle)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        print(f"[!] could not parse {root_manifest.name}: {exc}")
        return [f"unparseable root Cargo.toml: {exc}"]

    members = manifest.get("workspace", {}).get("members")
    if not isinstance(members, list) or not members:
        print("[!] root Cargo.toml has no [workspace] members list.")
        return ["no [workspace] members in root Cargo.toml"]

    declared = expand_members([str(m) for m in members])
    actual = find_crate_dirs()

    for name in sorted(actual - declared):
        message = f"directory '{name}/' has a Cargo.toml but is NOT in workspace members"
        print(f"[warning] {message}")
        warnings.append(message)
    for name in sorted(declared - actual):
        message = f"workspace member '{name}' has no matching directory/Cargo.toml"
        print(f"[warning] {message}")
        warnings.append(message)

    if not warnings:
        print(f"OK, {len(actual)} crate dir(s) match the workspace members.")
    return warnings


def find_nested_git() -> list[Path]:
    """Recursively find `.git` entries anywhere below the repo root.

    The repo's own top-level `.git` is excluded. A `.git` can be either a
    directory (normal clone/init) or a file (worktrees, submodules), so both
    forms are reported. The scan covers every directory, which is a superset
    of the `ch*` chapter folders.
    """
    findings: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(REPO_ROOT):
        if Path(dirpath) == REPO_ROOT:
            # Our own metadata: skip it, but keep scanning everything else.
            dirnames[:] = [d for d in dirnames if d != ".git"]
            continue
        if ".git" in dirnames:
            findings.append(Path(dirpath) / ".git")
            # No point scanning inside the nested repository's metadata.
            dirnames.remove(".git")
        if ".git" in filenames:
            findings.append(Path(dirpath) / ".git")
    return sorted(set(findings))


def check_git(check_only: bool) -> list[Path]:
    """Entry point for the --git check."""
    print("== check: nested .git directories ==")
    findings = find_nested_git()
    if not findings:
        print("OK, no nested .git found.")
        return []
    return offer_deletion(findings, "nested git repositor(ies)", check_only)


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="doctor.py",
        description="Health checker for the rustlearning workspace: stray "
        "build artifacts, workspace member mismatches, nested git repos.",
        epilog="With no selector flags all three checks run in order "
        "(binary, workspace, git). Deletion prompts default to No.",
    )
    parser.add_argument(
        "--binary",
        action="store_true",
        help="find and optionally delete stray compiled artifacts "
        "(e.g. from a direct `rustc main.rs` inside src/)",
    )
    parser.add_argument(
        "--workspace",
        action="store_true",
        help="warn if workspace members and crate directories disagree",
    )
    parser.add_argument(
        "--git",
        action="store_true",
        help="find and optionally delete nested .git repositories",
    )
    parser.add_argument(
        "--check-only",
        action="store_true",
        help="list findings without offering or performing any deletion",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)

    # No selector flags means: run everything, in a fixed sensible order.
    run_all = not (args.binary or args.workspace or args.git)

    remaining = 0

    if run_all or args.binary:
        remaining += len(check_binaries(args.check_only))
    if run_all or args.workspace:
        check_workspace()  # warnings only, does not affect the exit status
    if run_all or args.git:
        remaining += len(check_git(args.check_only))

    print("\n== summary ==")
    if remaining:
        print(f"[!] {remaining} issue(s) still on disk.")
        return 1
    print("All requested checks done, nothing harmful remains.")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("\n[i] interrupted, nothing deleted on this run.")
        sys.exit(2)
