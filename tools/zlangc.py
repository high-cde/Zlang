#!/usr/bin/env python3
"""Compilatore host di Zlang per il bytecode ZLB0 v1 di ZDOS x86_64."""

from __future__ import annotations

import argparse
import pathlib
import sys

MAGIC = b"ZLB0"
VERSION = 1
OP_EMIT = 0x01
OP_HALT = 0xFF


def compile_source(source: str, source_name: str) -> bytes:
    bytecode = bytearray(MAGIC)
    bytecode.append(VERSION)

    for line_number, raw_line in enumerate(source.splitlines(), start=1):
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        if line == "emit":
            raise ValueError(f"{source_name}:{line_number}: 'emit' richiede testo")
        if not line.startswith("emit "):
            raise ValueError(
                f"{source_name}:{line_number}: sintassi non supportata; "
                "il profilo ZLB0 v1 accetta solo 'emit <testo>'"
            )
        text = line[5:].encode("utf-8")
        if not text:
            raise ValueError(f"{source_name}:{line_number}: 'emit' richiede testo")
        if len(text) > 0xFFFF:
            raise ValueError(f"{source_name}:{line_number}: testo troppo lungo")
        bytecode.append(OP_EMIT)
        bytecode.extend(len(text).to_bytes(2, "little"))
        bytecode.extend(text)

    bytecode.append(OP_HALT)
    return bytes(bytecode)


def write_c_header(bytecode: bytes, destination: pathlib.Path) -> None:
    values = ", ".join(f"0x{value:02x}" for value in bytecode)
    destination.write_text(
        "#ifndef ZDOS_ZLANG_PROGRAM_H\n"
        "#define ZDOS_ZLANG_PROGRAM_H\n\n"
        "#include <stddef.h>\n"
        "#include <stdint.h>\n\n"
        f"static const uint8_t zlang_program[] = {{ {values} }};\n"
        "static const size_t zlang_program_length = sizeof(zlang_program);\n\n"
        "#endif\n",
        encoding="utf-8",
    )


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Compila il profilo Zlang ZLB0 v1 in un header C per ZDOS x86_64."
    )
    parser.add_argument("source", type=pathlib.Path, help="file sorgente .zlang")
    parser.add_argument("--header", type=pathlib.Path, required=True, help="header C di destinazione")
    parser.add_argument("--bytecode", type=pathlib.Path, help="file bytecode opzionale di destinazione")
    args = parser.parse_args()

    try:
        source = args.source.read_text(encoding="utf-8")
        bytecode = compile_source(source, str(args.source))
        args.header.parent.mkdir(parents=True, exist_ok=True)
        write_c_header(bytecode, args.header)
        if args.bytecode:
            args.bytecode.parent.mkdir(parents=True, exist_ok=True)
            args.bytecode.write_bytes(bytecode)
    except (OSError, UnicodeError, ValueError) as error:
        print(f"zlangc: errore: {error}", file=sys.stderr)
        return 1

    print(f"zlangc: compilato {args.source} ({len(bytecode)} byte ZLB0 v1)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
