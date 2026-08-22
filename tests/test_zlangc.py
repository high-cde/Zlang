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
    def compile_source(self, source: str) -> bytes:
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            source_path = root / "program.zlang"
            bytecode_path = root / "program.zlb"
            header_path = root / "zlang_program.h"
            source_path.write_text(source, encoding="utf-8")
            zlangc.compile_source(source_path, bytecode_path, header_path)
            return bytecode_path.read_bytes()

    def test_emit_compiles_to_zlb2(self) -> None:
        bytecode = self.compile_source("emit Ciao ZDOS\n")
        self.assertEqual(bytecode[:6], b"ZLB2\x02\x05")
        self.assertIn(b"Ciao ZDOS", bytecode)
        self.assertEqual(bytecode[-3:], b"\xff\x00\x00")

    def test_comments_and_empty_lines_are_ignored(self) -> None:
        bytecode = self.compile_source("\n# nota\nemit ok\n")
        self.assertEqual(bytecode[:6], b"ZLB2\x02\x05")
        self.assertIn(b"ok", bytecode)

    def test_let_is_encoded_as_zlb2_record(self) -> None:
        bytecode = self.compile_source("let risposta = 42\n")
        self.assertEqual(bytecode[:6], b"ZLB2\x02\x05")
        self.assertIn(b"risposta = 42", bytecode)
        self.assertEqual(bytecode[-3:], b"\xff\x00\x00")

    def test_unknown_statement_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            source = pathlib.Path(directory) / "program.zlang"
            bytecode = pathlib.Path(directory) / "program.zlb"
            header = pathlib.Path(directory) / "program.h"
            source.write_text("unknown instruction\n", encoding="utf-8")
            with self.assertRaises(SystemExit):
                zlangc.compile_source(source, bytecode, header)

    def test_header_exposes_zlb2_bytecode(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            source = root / "program.zlang"
            bytecode = root / "program.zlb"
            header = root / "zlang_program.h"
            source.write_text("emit ok\n", encoding="utf-8")
            zlangc.compile_source(source, bytecode, header)
            text = header.read_text(encoding="utf-8")
        self.assertIn("zlang_bytecode", text)
        self.assertIn("0x5a", text)
        self.assertIn("0x32", text)


if __name__ == "__main__":
    unittest.main()
