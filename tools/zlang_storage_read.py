#!/usr/bin/env python3
"""Execute the Zlang storage.read-v1 subset in a confined Linux namespace."""
from __future__ import annotations

import argparse
import hashlib
import re
import sys
from pathlib import Path

READ_RE = re.compile(r'^storage\.read\s+["\']([^"\']+)["\']$')


def safe_path(root: Path, relative: str) -> Path:
    candidate = Path(relative)
    if candidate.is_absolute() or "\x00" in relative or ".." in candidate.parts:
        raise ValueError("storage.read path escapes namespace")
    resolved_root = root.resolve()
    resolved = (resolved_root / candidate).resolve()
    if resolved != resolved_root and resolved_root not in resolved.parents:
        raise ValueError("storage.read path escapes namespace")
    return resolved


def main() -> int:
    parser = argparse.ArgumentParser(description="Run Zlang storage.read-v1 in a read-only namespace")
    parser.add_argument("source")
    parser.add_argument("--root", required=True, help="authorized namespace root, e.g. /mnt/data")
    parser.add_argument("--max-bytes", type=int, default=4096)
    args = parser.parse_args()
    if args.max_bytes < 1 or args.max_bytes > 1024 * 1024:
        parser.error("--max-bytes must be between 1 and 1048576")

    root = Path(args.root)
    if not root.is_dir():
        print("storage.read: namespace unavailable", file=sys.stderr)
        return 2

    for raw in Path(args.source).read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        match = READ_RE.fullmatch(line)
        if not match:
            print(f"storage.read-v1: unsupported instruction -> {line!r}", file=sys.stderr)
            return 3
        try:
            target = safe_path(root, match.group(1))
        except ValueError as exc:
            print(f"storage.read-v1: denied: {exc}", file=sys.stderr)
            return 4
        if not target.is_file():
            print("storage.read-v1: file not found", file=sys.stderr)
            return 5
        data = target.read_bytes()
        if len(data) > args.max_bytes:
            print("storage.read-v1: read quota exceeded", file=sys.stderr)
            return 6
        sys.stdout.buffer.write(data)
        if data and not data.endswith(b"\n"):
            sys.stdout.buffer.write(b"\n")
        print(f"ZLANG_STORAGE_READ_OK path={match.group(1)} bytes={len(data)} sha256={hashlib.sha256(data).hexdigest()}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

