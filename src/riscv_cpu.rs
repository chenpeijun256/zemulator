
// enum instr_type {
//     IS_U,
//     IS_J,
//     IS_I,
//     IS_B,
//     IS_S,
//     IS_R,
// }

// enum instr_type {
//     IS_LUI,
//     IS_AUIPC,
//     IS_JAL,
//     IS_JALR,
//     IS_JB,
//     IS_LOAD,
//     IS_STORE,
//     IS_MATH_I,
//     IS_MATH,
//     IS_FENCE,
//     IS_CSR,
//     IS_SYS,
// }

const REG_NUM: usize = 32;

pub struct RiscvCpu {
    pc: u32,
    reg: [u32; REG_NUM]
}

impl RiscvCpu {
    pub fn new(reset_pc: u32) -> Self {
        RiscvCpu{ pc: reset_pc, reg: [0; REG_NUM]}
    }

    pub fn tick(&mut self, instr:u32) -> u32 {
        //opcode = instr[6:0];
        match instr & 0x7f {
            //lui 7'b0110111
            0x37 => {
                self.execute_lui(instr);
                self.pc += 4;
            },
            //auipc 7'b0010111
            0x17 => {
                self.execute_auipc(instr);
                self.pc += 4;
            },
            //jal 7'b1101111
            0x6f => {
                self.execute_jal(instr);
            },
            //jalr = (opcode == 7'b1100111);
            0x67 => {
                self.execute_jalr(instr);
            },
            //jb  7'b1100011
            0x63 => {
                self.execute_jb(instr);
            },
            //math i 7'b0010011
            0x13 => {
                self.execute_math_i(instr);
                self.pc += 4;
            },
            //math 7'b0110011
            0x33 => {
                self.execute_math(instr);
                self.pc += 4;
            },
            //others
            _ => {
                println!("illegal instruction. {:08x}.", instr);
                self.pc += 4;
            },
        }

        return self.pc;
    }

    fn set_rd(&mut self, instr: u32, data: u32) -> usize {
        let rd = (instr>>7 & 0x1f) as usize;
        if rd != 0 {
            self.reg[rd] = data;
        }
        rd
    }

    fn get_rs_1(&mut self, instr: u32) -> (usize, u32) {
        let r1 = (instr>>15 & 0x1f) as usize;
        if r1 ==0 {
            return (0, 0);
        }
        (r1, self.reg[r1])
    }

    fn get_rs_2(&mut self, instr: u32) -> (usize, u32) {
        let r2 = (instr>>20 & 0x1f) as usize;
        if r2 ==0 {
            return (0, 0);
        }
        (r2, self.reg[r2])
    }

    fn execute_lui(&mut self, instr: u32) {
        let imm = instr & 0xfffff000;
        let rd = self.set_rd(instr, imm);
        println!("lui x{rd}, {:x}", imm);
    }

    fn execute_auipc(&mut self, instr: u32) {
        let imm = instr & 0xfffff000;
        let rd = self.set_rd(instr, imm + self.pc);
        println!("auipc x{rd}, {:x}", imm);
    }

    fn execute_jal(&mut self, instr: u32) {
        let imm = (instr & 0x000ff000) | 
                    ((instr>>8) & 0x00000800) | 
                    ((instr>>20) & 0x000007fe);
        let offset = if instr & 0x80000000 == 0x80000000 {0xfff00000 | imm } else { imm };
        let rd = self.set_rd(instr, self.pc + 4);
        self.pc += offset;
        println!("jal x{rd}, {offset}");
    }

    fn execute_jalr(&mut self, instr: u32) {
        let imm = (instr>>20) & 0x00000fff;
        let offset = if instr & 0x80000000 == 0x80000000 { 0xfffff000 | imm } else { imm };
        let rd = self.set_rd(instr, self.pc + 4);
        let (rs1, r1_data) = self.get_rs_1(instr);
        self.pc = (r1_data + offset) & 0xfffffffe;
        println!("jalr x{rd}, {offset}(x{rs1})");
    }

    fn execute_jb(&mut self, instr: u32) {
        let (rs1, rs1_data) = self.get_rs_1(instr);
        let (rs2, rs2_data) = self.get_rs_2(instr);
        let imm = ((instr<<4) & 0x00000800) | 
                    ((instr>>20) & 0x000007e0) | 
                    ((instr>>7) & 0x0000001e);
        let offset = if instr & 0x80000000 == 0x80000000 {0xfffff000 | imm } else { imm };
        match instr>>12 & 0x07 {
            //beq 3'b000
            0x00 => {
                if rs1_data == rs2_data { 
                    self.pc += offset; 
                } else {
                    self.pc += 4; 
                }
                println!("beq x{rs1}, x{rs2}, {offset}");
            },
            //bne 3'b001
            0x01 => {
                if rs1_data != rs2_data { 
                    self.pc += offset; 
                    println!("bne pc: {:x}", self.pc);
                } else {
                    self.pc += 4; 
                }
                println!("bne x{rs1}, x{rs2}, {offset}");
            },
            //blt 3'b100
            0x04 => {
                if (rs1_data as i32) < (rs2_data as i32) { 
                    self.pc += offset; 
                } else {
                    self.pc += 4; 
                }
                println!("blt x{rs1}, x{rs2}, {offset}");
            },
            //bge 3'b101
            0x05 => {
                if (rs1_data as i32) >= (rs2_data as i32) { 
                    self.pc += offset; 
                } else {
                    self.pc += 4; 
                }
                println!("bge x{rs1}, x{rs2}, {offset}");
            },
            //bltu 3'b110
            0x06 => {
                if rs1_data < rs2_data { 
                    self.pc += offset; 
                } else {
                    self.pc += 4; 
                }
                println!("bltu x{rs1}, x{rs2}, {offset}");
            },
            //bgeu 3'b111
            0x07 => {
                if rs1_data >= rs2_data { 
                    self.pc += offset; 
                } else {
                    self.pc += 4; 
                }
                println!("bgeu x{rs1}, x{rs2}, {offset}");
            },
            //others
            _ => println!("jb illegal instruction. {:08x}.", instr),
        }
    }

    fn execute_math_i(&mut self, instr: u32) {
        let (rs1, rs1_data) = self.get_rs_1(instr);
        let imm = (instr>>20) & 0x00000fff;
        let s_imm = if instr & 0x80000000 == 0x80000000 { 0xfffff000 | imm } else { imm };
        // let s_imm = e_imm as i32;

        match instr>>12 & 0x07 {
            //addi 3'b000
            0x00 => {
                let rd = self.set_rd(instr, rs1_data + s_imm);
                println!("addi x{rd}, x{rs1}, {s_imm}");
            },
            //others
            _ => println!("math i illegal instruction. {:08x}.", instr),
        }
    }

    fn execute_math(&mut self, instr: u32) {
        let (rs1, rs1_data) = self.get_rs_1(instr);
        let (rs2, rs2_data) = self.get_rs_2(instr);

        match instr>>12 & 0x07 {
            //addi 3'b000
            0x00 => {
                match instr>>25 & 0x7f {
                    0x00 => {
                        let rd = self.set_rd(instr, rs1_data + rs2_data);
                        println!("add x{rd}, x{rs1}, x{rs2}");
                    },
                    0x20 => {
                        let rd = self.set_rd(instr, rs1_data - rs2_data);
                        println!("sub x{rd}, x{rs1}, x{rs2}");
                    }
                    _ => {
                        println!("math add sub illegal instruction. {:08x}.", instr)
                    },
                }
                
            },
            //others
            _ => println!("math i illegal instruction. {:08x}.", instr),
        }
    }
}