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
    def compile_source(self, source: str, *, target: str = "linux-bridge") -> bytes:
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            source_path = root / "program.zlang"
            bytecode_path = root / "program.zlb"
            header_path = root / "zlang_program.h"
            source_path.write_text(source, encoding="utf-8")
            zlangc.compile_source(source_path, bytecode_path, header_path, target=target)
            return bytecode_path.read_bytes()

    def test_emit_compiles_to_zlb2(self) -> None:
        bytecode = self.compile_source("emit Ciao ZDOS\n", target="zdos-x86_64")
        self.assertEqual(bytecode[:6], b"ZLB2\x02\x05")
        self.assertIn(b"Ciao ZDOS", bytecode)
        self.assertEqual(bytecode[-3:], b"\xff\x00\x00")

    def test_comments_and_empty_lines_are_ignored(self) -> None:
        bytecode = self.compile_source("\n# nota\nemit ok\n", target="zdos-x86_64")
        self.assertEqual(bytecode[:6], b"ZLB2\x02\x05")
        self.assertIn(b"ok", bytecode)

    def test_unsupported_language_constructs_are_rejected(self) -> None:
        for source in ("let risposta = 42\n", "if ok\n", "loop:\n", "wait\n"):
            with self.subTest(source=source):
                with self.assertRaises(zlangc.CompileError):
                    self.compile_source(source)

    def test_storage_read_is_only_available_to_linux_bridge(self) -> None:
        bytecode = self.compile_source('storage.read "data.txt"\n', target="linux-bridge")
        self.assertIn(b"\x06\x08\x00data.txt", bytecode)
        with self.assertRaises(zlangc.CompileError):
            self.compile_source('storage.read "data.txt"\n', target="zdos-x86_64")

    def test_unknown_statement_is_rejected(self) -> None:
        with self.assertRaises(zlangc.CompileError):
            self.compile_source("unknown instruction\n")

    def test_header_exposes_bytecode_and_size(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            source = root / "program.zlang"
            bytecode = root / "program.zlb"
            header = root / "zlang_program.h"
            source.write_text("emit ok\n", encoding="utf-8")
            zlangc.compile_source(source, bytecode, header, target="zdos-x86_64")
            text = header.read_text(encoding="utf-8")
        self.assertIn("zlang_bytecode", text)
        self.assertIn("zlang_bytecode_size", text)
        self.assertIn("0x5a", text)
        self.assertIn("0x32", text)


if __name__ == "__main__":
    unittest.main()
