import sys
import argparse

def compile_source(source_path, bytecode_path, header_path):
    with open(source_path, 'r') as f:
        lines = f.readlines()

    bytecode = bytearray()
    bytecode.extend(b'ZLB2') # Magic v2.0+
    bytecode.extend(b'\x02\x05') # Versione 2.5 (Aritmetica attiva)

    instructions_compiled = 0

    for line in lines:
        line = line.strip()
        if not line or line.startswith('#'):
            continue
        
        if line.startswith('emit '):
            text = line[5:].strip().strip('"\'')
            payload = text.encode('utf-8')
            bytecode.append(0x01) # EMIT
            bytecode.extend(len(payload).to_bytes(2, byteorder='little'))
            bytecode.extend(payload)
            instructions_compiled += 1
            
        elif line.startswith('let '):
            # Sintassi supportate:
            # let x = 100
            # let result = 50 + 25
            content = line[4:].strip()
            payload = content.encode('utf-8')
            bytecode.append(0x02) # VIRTUAL RAM / ALU ASSIGN
            bytecode.extend(len(payload).to_bytes(2, byteorder='little'))
            bytecode.extend(payload)
            instructions_compiled += 1

        elif line.startswith('if '):
            # Sintassi: if x == 100 jump label
            payload = line[3:].encode('utf-8')
            bytecode.append(0x03) # CONDITIONAL JUMP
            bytecode.extend(len(payload).to_bytes(2, byteorder='little'))
            bytecode.extend(payload)
            instructions_compiled += 1

        elif line.endswith(':'):
            label_name = line[:-1].encode('utf-8')
            bytecode.append(0x04) # LABEL MARKER
            bytecode.extend(len(label_name).to_bytes(2, byteorder='little'))
            bytecode.extend(label_name)
            instructions_compiled += 1

        elif line.startswith('wait'):
            payload = b'async_signal'
            bytecode.append(0x05) # ASYNC INTERRUPT
            bytecode.extend(len(payload).to_bytes(2, byteorder='little'))
            bytecode.extend(payload)
            instructions_compiled += 1
        else:
            print(f"zlangc v2.5 error: sintassi sconosciuta -> '{line}'")
            sys.exit(1)

    bytecode.append(0xff) # HALT
    bytecode.extend((0).to_bytes(2, byteorder='little'))

    with open(bytecode_path, 'wb') as f:
        f.write(bytecode)

    with open(header_path, 'w') as f:
        f.write("/* Z-LANG V2.5 ALU HEADER */\n")
        f.write("#ifndef ZLANG_PROGRAM_H\n#define ZLANG_PROGRAM_H\n\n")
        f.write(f"static const unsigned char zlang_bytecode[] = {{\n    ")
        f.write(", ".join(hex(b) for b in bytecode))
        f.write("\n};\n\n#endif\n")

    print(f"zlangc v2.5: compilato {source_path} ({len(bytecode)} byte, {instructions_compiled} istruzioni)")

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Z-Lang Compiler v2.5')
    parser.add_argument('source', help='Sorgente .zlang')
    parser.add_argument('--bytecode', required=True)
    parser.add_argument('--header', required=True)
    args = parser.parse_args()
    compile_source(args.source, args.bytecode, args.header)
