#!/usr/bin/env python3
"""Fail the release gate when generated artifacts are tracked by Git."""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[1]

FORBIDDEN_PREFIXES = ("target/",)
FORBIDDEN_PARTS = {"node_modules", "__pycache__", ".pytest_cache"}
FORBIDDEN_SUFFIXES = (".pyc", ".pyo", ".iso", ".img", ".zlb")


def is_forbidden(path: str) -> bool:
    candidate = PurePosixPath(path)
    return (
        path.startswith(FORBIDDEN_PREFIXES)
        or any(part in FORBIDDEN_PARTS for part in candidate.parts)
        or path.endswith(FORBIDDEN_SUFFIXES)
    )


def main() -> int:
    result = subprocess.run(
        ["git", "ls-files"],
        capture_output=True,
        text=True,
        check=False,
        cwd=ROOT,
    )
    if result.returncode:
        print("ZLANG_HYGIENE_FAILED: impossibile leggere l'indice Git", file=sys.stderr)
        return result.returncode

    forbidden = [path for path in result.stdout.splitlines() if is_forbidden(path)]
    if forbidden:
        print("ZLANG_HYGIENE_FAILED: artefatti generati tracciati:", file=sys.stderr)
        print("\n".join(forbidden), file=sys.stderr)
        return 1

    checks = ("target/.sentinel", "tests/__pycache__/.sentinel", "tools/__pycache__/.sentinel")
    for ignored in checks:
        probe = subprocess.run(
            ["git", "check-ignore", "-q", "--no-index", ignored],
            check=False,
            cwd=ROOT,
        )
        if probe.returncode != 0:
            print(f"ZLANG_HYGIENE_FAILED: {ignored} non è ignorato", file=sys.stderr)
            return 1

    print("ZLANG_HYGIENE_OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
