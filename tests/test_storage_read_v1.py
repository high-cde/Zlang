from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
COMPILER = ROOT / "tools" / "zlangc.py"
BRIDGE = ROOT / "tools" / "zlang_storage_read.py"


def test_compiler_emits_storage_read_opcode(tmp_path: Path) -> None:
    source = tmp_path / "program.zlang"
    bytecode = tmp_path / "program.zlb"
    header = tmp_path / "program.h"
    source.write_text('storage.read "data.txt"\n')
    subprocess.run([sys.executable, str(COMPILER), str(source), "--bytecode", str(bytecode), "--header", str(header)], check=True)
    data = bytecode.read_bytes()
    assert data[:6] == b"ZLB2\x02\x05"
    assert b"\x06\x08\x00data.txt" in data


def test_bridge_reads_only_inside_namespace(tmp_path: Path) -> None:
    namespace = tmp_path / "data"
    namespace.mkdir()
    (namespace / "marker").write_text("persistent\n")
    source = tmp_path / "read.zlang"
    source.write_text('storage.read "marker"\n')
    result = subprocess.run([sys.executable, str(BRIDGE), str(source), "--root", str(namespace)], capture_output=True, text=True, check=True)
    assert result.stdout == "persistent\n"
    assert "ZLANG_STORAGE_READ_OK path=marker" in result.stderr


def test_bridge_denies_escape(tmp_path: Path) -> None:
    namespace = tmp_path / "data"
    namespace.mkdir()
    source = tmp_path / "escape.zlang"
    source.write_text('storage.read "../secret"\n')
    result = subprocess.run([sys.executable, str(BRIDGE), str(source), "--root", str(namespace)], capture_output=True, text=True)
    assert result.returncode == 4
    assert "denied" in result.stderr
