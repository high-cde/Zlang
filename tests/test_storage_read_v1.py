from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
COMPILER = ROOT / "tools" / "zlangc.py"
BRIDGE = ROOT / "tools" / "zlang_storage_read.py"


class StorageReadV1Tests(unittest.TestCase):
    def run_compiler(self, source: Path, bytecode: Path, header: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [
                sys.executable,
                str(COMPILER),
                str(source),
                "--bytecode",
                str(bytecode),
                "--header",
                str(header),
                "--target",
                "linux-bridge",
            ],
            capture_output=True,
            text=True,
            check=False,
        )

    def test_compiler_emits_storage_read_opcode(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "program.zlang"
            bytecode = root / "program.zlb"
            header = root / "program.h"
            source.write_text('storage.read "data.txt"\n', encoding="utf-8")
            result = self.run_compiler(source, bytecode, header)
            self.assertEqual(result.returncode, 0, result.stderr)
            data = bytecode.read_bytes()
        self.assertEqual(data[:6], b"ZLB2\x02\x05")
        self.assertIn(b"\x06\x08\x00data.txt", data)

    def test_bridge_reads_only_inside_namespace(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            namespace = root / "data"
            namespace.mkdir()
            (namespace / "marker").write_text("persistent\n", encoding="utf-8")
            source = root / "read.zlang"
            source.write_text('storage.read "marker"\n', encoding="utf-8")
            result = subprocess.run(
                [sys.executable, str(BRIDGE), str(source), "--root", str(namespace)],
                capture_output=True,
                text=True,
                check=False,
            )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout, "persistent\n")
        self.assertIn("ZLANG_STORAGE_READ_OK path=marker", result.stderr)

    def test_bridge_denies_escape(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            namespace = root / "data"
            namespace.mkdir()
            source = root / "escape.zlang"
            source.write_text('storage.read "../secret"\n', encoding="utf-8")
            result = subprocess.run(
                [sys.executable, str(BRIDGE), str(source), "--root", str(namespace)],
                capture_output=True,
                text=True,
                check=False,
            )
        self.assertEqual(result.returncode, 4)
        self.assertIn("denied", result.stderr)


if __name__ == "__main__":
    unittest.main()
