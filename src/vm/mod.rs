pub mod cpu;

use std::collections::HashMap;
use cpu::VirtualCPU;

pub struct ZVirtualMachine {
    pub memory: HashMap<String, String>,
    pub cpu: VirtualCPU,
}

impl Default for ZVirtualMachine {
    fn default() -> Self { Self::new() }
}

impl ZVirtualMachine {
    pub fn new() -> Self {
        ZVirtualMachine {
            memory: HashMap::new(),
            cpu: VirtualCPU::new(4096),
        }
    }

    pub fn execute(&mut self, bytecode: Vec<String>) -> String {
        let mut output = String::new();
        output.push_str("[ZDOS vCPU] Inizializzazione ecosistema e State Trie vRAM...\n");

        let mut raw_program = Vec::new();
        let mut reg_counter: u8 = 1;
        let mut var_map: HashMap<String, u8> = HashMap::new();
        let mut mem_addrs: HashMap<String, u16> = HashMap::new();
        let mut next_addr: u16 = 0x0100; // Indirizzo vRAM iniziale per lo storage

        for instr in &bytecode {
            if let Some(stripped) = instr.strip_prefix("PRINT_STDOUT ") {
                output.push_str(&format!("[ZLang VM] {}\n", stripped));
            } else if instr.starts_with("LET_ASSIGN ") {
                let assignment = &instr[11..];
                if assignment.contains('-') {
                    // Gestione operazione di sottrazione (es. balance - transfer)
                    let parts: Vec<&str> = assignment.split('=').collect();
                    if parts.len() == 2 {
                        let dest_var = parts[0].trim();
                        let expr: Vec<&str> = parts[1].split('-').map(|s| s.trim()).collect();
                        if expr.len() == 2 {
                            let v1 = expr[0];
                            let v2 = expr[1];
                            let r_dest = *var_map.entry(dest_var.to_string()).or_insert_with(|| { let r = reg_counter; reg_counter = (reg_counter % 14) + 1; r });
                            let r1 = *var_map.get(v1).unwrap_or(&1);
                            let r2 = *var_map.get(v2).unwrap_or(&2);

                            // Opcode 0x04: SUB r_dest, r1, r2
                            raw_program.push(0x04);
                            raw_program.push(r_dest);
                            raw_program.push(r1);
                            raw_program.push(r2);

                            // Salviamo in vRAM
                            let addr = *mem_addrs.entry(dest_var.to_string()).or_insert_with(|| { let a = next_addr; next_addr += 8; a });
                            raw_program.push(0x05);
                            raw_program.extend_from_slice(&addr.to_le_bytes());
                            raw_program.push(r_dest);

                            raw_program.push(0x01);
                            raw_program.push(r_dest);
                        }
                    }
                } else {
                    // Assegnazione standard (valore immediato)
                    let parts: Vec<&str> = assignment.split('=').collect();
                    if parts.len() == 2 {
                        let var_name = parts[0].trim().to_string();
                        let val: u64 = parts[1].trim().parse().unwrap_or(0);
                        
                        let reg = *var_map.entry(var_name.clone()).or_insert_with(|| {
                            let r = reg_counter;
                            reg_counter = (reg_counter % 14) + 1;
                            r
                        });

                        let addr = *mem_addrs.entry(var_name).or_insert_with(|| {
                            let a = next_addr;
                            next_addr += 8;
                            a
                        });

                        // Caricamento immediato (LOAD IMM)
                        raw_program.push(0x02);
                        raw_program.push(reg);
                        raw_program.extend_from_slice(&val.to_le_bytes());

                        // Salvataggio in vRAM State Trie
                        raw_program.push(0x05);
                        raw_program.extend_from_slice(&addr.to_le_bytes());
                        raw_program.push(reg);

                        // Emetti valore
                        raw_program.push(0x01);
                        raw_program.push(reg);
                    }
                }
            }
        }

        raw_program.push(0xFF); // HALT

        self.cpu.load_program(&raw_program);
        self.cpu.pc = 0;
        self.cpu.halted = false;

        let mut cycles = 0;
        while !self.cpu.halted && cycles < 3000 {
            match self.cpu.step() {
                Ok(Some(msg)) => output.push_str(&format!("{}\n", msg)),
                Ok(None) => {}
                Err(e) => { output.push_str(&format!("[Errore vCPU] {}\n", e)); break; }
            }
            cycles += 1;
        }

        output.push_str("[ZDOS vCPU] Esecuzione transazione State Trie completata.\n");
        output
    }
}
