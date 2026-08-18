from __future__ import annotations

import importlib.util
import pathlib
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "zlangc.py"
SPEC = importlib.util.spec_from_file_location("zlangc", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
zlangc = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(zlangc)


class ZlangCompilerTests(unittest.TestCase):
    def test_emit_compiles_to_versioned_zlb0(self) -> None:
        bytecode = zlangc.compile_source("emit Ciao ZDOS", "memory.zlang")
        self.assertEqual(
            bytecode,
            b"ZLB0" + bytes([1, 1, 9, 0]) + b"Ciao ZDOS" + bytes([0xFF]),
        )

    def test_comments_and_empty_lines_are_ignored(self) -> None:
        bytecode = zlangc.compile_source("\n# nota\nemit ok\n", "memory.zlang")
        self.assertEqual(bytecode, b"ZLB0" + bytes([1, 1, 2, 0]) + b"ok" + bytes([0xFF]))

    def test_unsupported_statement_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "sintassi non supportata"):
            zlangc.compile_source("let x = 1", "memory.zlang")

    def test_empty_emit_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "richiede testo"):
            zlangc.compile_source("emit ", "memory.zlang")

    def test_header_exposes_program_and_length(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            destination = pathlib.Path(directory) / "program.h"
            zlangc.write_c_header(b"ZLB0\x01\xff", destination)
            header = destination.read_text(encoding="utf-8")
        self.assertIn("zlang_program", header)
        self.assertIn("zlang_program_length", header)
        self.assertIn("0x5a", header)


if __name__ == "__main__":
    unittest.main()
