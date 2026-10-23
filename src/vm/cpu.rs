pub struct VirtualCPU {
    pub registers: [u64; 16], // r0-r15
    pub pc: usize,            // Program Counter
    pub sp: usize,            // Stack Pointer
    pub flags: u32,           // Bit 0: Zero flag
    pub memory: Vec<u8>,      // vRAM lineare (4KB)
    pub halted: bool,
}

impl VirtualCPU {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: [0; 16],
            pc: 0,
            sp: memory_size - 1,
            flags: 0,
            memory: vec![0; memory_size],
            halted: false,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        let len = program.len();
        if len <= self.memory.len() {
            self.memory[..len].copy_from_slice(program);
        }
    }

    pub fn step(&mut self) -> Result<Option<String>, String> {
        if self.pc >= self.memory.len() {
            return Err("Segmentation Fault: PC out of bounds".to_string());
        }

        let opcode = self.memory[self.pc];
        self.pc += 1;

        match opcode {
            0x01 => { // EMIT / PRINT REG
                let reg_id = self.fetch_u8()?;
                let val = self.registers[reg_id as usize];
                Ok(Some(format!("[vCPU OUT] Registro R{} = {}", reg_id, val)))
            }
            0x02 => { // LOAD IMMEDIATE: R_dest = value
                let reg_id = self.fetch_u8()?;
                let val = self.fetch_u64()?;
                self.registers[reg_id as usize] = val;
                Ok(Some(format!("[vCPU] R{} = {} (LOAD IMM)", reg_id, val)))
            }
            0x03 => { // ADD: R_dest = R_src1 + R_src2
                let dest = self.fetch_u8()? as usize;
                let src1 = self.fetch_u8()? as usize;
                let src2 = self.fetch_u8()? as usize;
                self.registers[dest] = self.registers[src1].wrapping_add(self.registers[src2]);
                self.update_zero_flag(self.registers[dest]);
                Ok(Some(format!("[vCPU] R{} = R{} + R{} (Valore: {})", dest, src1, src2, self.registers[dest])))
            }
            0x04 => { // SUB: R_dest = R_src1 - R_src2
                let dest = self.fetch_u8()? as usize;
                let src1 = self.fetch_u8()? as usize;
                let src2 = self.fetch_u8()? as usize;
                self.registers[dest] = self.registers[src1].wrapping_sub(self.registers[src2]);
                self.update_zero_flag(self.registers[dest]);
                Ok(Some(format!("[vCPU] R{} = R{} - R{} (Valore: {})", dest, src1, src2, self.registers[dest])))
            }
            0x05 => { // STORE_MEM: Salva registro in vRAM all'indirizzo 16-bit
                let addr = self.fetch_u16()? as usize;
                let reg_id = self.fetch_u8()? as usize;
                let val = self.registers[reg_id];
                if addr + 8 <= self.memory.len() {
                    self.memory[addr..addr + 8].copy_from_slice(&val.to_le_bytes());
                    Ok(Some(format!("[vCPU vRAM] Salvato R{} ({}) all'indirizzo vRAM 0x{:04X}", reg_id, val, addr)))
                } else {
                    Err("vRAM Out of bounds store".to_string())
                }
            }
            0x06 => { // LOAD_MEM: Carica da vRAM in un registro
                let reg_id = self.fetch_u8()? as usize;
                let addr = self.fetch_u16()? as usize;
                if addr + 8 <= self.memory.len() {
                    let bytes: [u8; 8] = self.memory[addr..addr + 8].try_into().unwrap();
                    let val = u64::from_le_bytes(bytes);
                    self.registers[reg_id] = val;
                    Ok(Some(format!("[vCPU vRAM] Caricato valore {} da vRAM 0x{:04X} in R{}", val, addr, reg_id)))
                } else {
                    Err("vRAM Out of bounds load".to_string())
                }
            }
            0xFF => { // HALT
                self.halted = true;
                Ok(Some("[vCPU] HALT raggiunto".to_string()))
            }
            _ => Err(format!("UNKNOWN_OPCODE: 0x{:02X} a PC=0x{:04X}", opcode, self.pc - 1)),
        }
    }

    fn fetch_u8(&mut self) -> Result<u8, String> {
        let val = self.memory[self.pc];
        self.pc += 1;
        Ok(val)
    }

    fn fetch_u16(&mut self) -> Result<u16, String> {
        let bytes: [u8; 2] = self.memory[self.pc..self.pc + 2].try_into().unwrap();
        self.pc += 2;
        Ok(u16::from_le_bytes(bytes))
    }

    fn fetch_u64(&mut self) -> Result<u64, String> {
        let bytes: [u8; 8] = self.memory[self.pc..self.pc + 8].try_into().unwrap();
        self.pc += 8;
        Ok(u64::from_le_bytes(bytes))
    }

    fn update_zero_flag(&mut self, val: u64) {
        if val == 0 { self.flags |= 1; } else { self.flags &= !1; }
    }
}
