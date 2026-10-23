import sys
import subprocess

if len(sys.argv) < 2:
    print("Uso: python3 tools/run_zlang.py <file.zl>")
    sys.exit(1)

source_file = sys.argv[1]
bytecode_file = source_file.replace(".zl", ".zbin")
header_file = source_file.replace(".zl", ".h")

print(f"[*] Compilazione di {source_file} tramite zlangc.py...")
subprocess.run([
    "python3", "tools/zlangc.py",
    "--bytecode", bytecode_file,
    "--header", header_file,
    source_file
], check=True)

print(f"[*] Esecuzione del bytecode {bytecode_file} con la VM ZLang...")
subprocess.run([
    "./target/release/zlang", bytecode_file
], check=True)
