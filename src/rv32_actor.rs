mod com_reg;
mod csr_reg;
mod riscv_cpu;

use crate::csr_reg::CsrReg;
use crate::perips::Perips;
use crate::com_reg::ComReg;
use crate::mem::Mem;
use crate::mem::MemIO;

pub struct Rv32Actor {
    name: String,
    
    tick_cnt: u32,

    pc: u32,

    reg: ComReg,
    csr: CsrReg,

    mems: Vec<Mem>,
    perips: Vec<Perips>,
}

impl RiscvCpu {
    pub fn new(name: String, rst_pc: u32) -> Self {
        RiscvCpu{
                    name,
                    pc: rst_pc, 
                    reg: ComReg::new(32), 
                    tick_cnt: 0, 
                    csr: CsrReg::new(), 
                    mems: Vec::new(),
                    perips: Vec::new(),
                }
    }

    pub fn add_mem(&mut self, mem: Mem) {
        self.mems.push(mem);
    }

    pub fn add_perips(&mut self, p: Perips) {
        self.perips.push(p);
    }

    pub fn fill_mem(&mut self, m_index: usize, data: Vec<u8>, pos: u32) {
        if m_index < self.mems.len() {
            self.mems[m_index].fill(data, pos);
        }
    }

    fn print_info(&mut self, instr: u32) {
        println!("({})tick: {}, pc: {:x}, instr: {:08x}", self.name, self.tick_cnt, self.pc, instr);
        // println!("{}", self.reg.to_string());
    }

    pub fn tick(&mut self) {
        self.tick_cnt += 1;

        let instr = self.mems[0].read_u32(self.pc);
        self.print_info(instr);

        //opcode = instr[6:0];
        match instr & 0x7f {
            //lui 7'b0110111
            0x37 => {
                self.execute_lui(instr); 
                self.pc = self.pc.wrapping_add(4);
            },
            //auipc 7'b0010111
            0x17 => {
                self.execute_auipc(instr);
                self.pc = self.pc.wrapping_add(4);
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
                self.pc = self.pc.wrapping_add(4);
            },
            //math 7'b0110011
            0x33 => {
                self.execute_math(instr);
                self.pc = self.pc.wrapping_add(4);
            },
            //load, 7'b0000011
            0x03 => {
                self.execute_load(instr);
                self.pc = self.pc.wrapping_add(4);
            },
            //store, 7'b0100011
            0x23 => {
                self.execute_store(instr);
                self.pc = self.pc.wrapping_add(4);
            },
            //fence  7'b0001111
            0x0f => {
                self.execute_fence(instr);
                self.pc = self.pc.wrapping_add(4);
            },
            //sys 7'b1110011
            0x73 => {
                self.execute_sys(instr);
                self.pc = self.pc.wrapping_add(4);
            },
            //others
            _ => {
                panic!("pc: {:x} inllegal instruction. {:08x}.", self.pc, instr);
            },
        }
    }

    pub fn get_tick_cnt(&self) -> u32 {
        self.tick_cnt
    }

    pub fn get_rs(&self, index: u32) -> u32 {
        self.reg.read(index)
    }

    fn set_rd(&mut self, instr: u32, data: u32) -> u32 {
        let rd = instr>>7 & 0x1f;
        if rd != 0 {
            self.reg.write(rd, data);
        }
        rd
    }

    fn get_rs_1(&self, instr: u32) -> (u32, u32) {
        let r1 = instr>>15 & 0x1f;
        if r1 == 0 {
            return (0, 0);
        }
        (r1, self.reg.read(r1))
    }

    fn get_rs_2(&mut self, instr: u32) -> (u32, u32) {
        let r2 = instr>>20 & 0x1f;
        if r2 == 0 {
            return (0, 0);
        }
        (r2, self.reg.read(r2))
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
        self.pc = self.pc.wrapping_add(offset);
        println!("jal x{rd}, {}", offset as i32);
    }

    fn execute_jalr(&mut self, instr: u32) {
        let imm = (instr>>20) & 0x00000fff;
        let offset = if instr & 0x80000000 == 0x80000000 { 0xfffff000 | imm } else { imm };
        let rd = self.set_rd(instr, self.pc + 4);
        let (rs1, r1_data) = self.get_rs_1(instr);
        self.pc = (r1_data.wrapping_add(offset)) & 0xfffffffe;
        println!("jalr x{rd}, {}(x{rs1})", offset as i32);
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
                    self.pc = self.pc.wrapping_add(offset); 
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
                println!("beq x{rs1}, x{rs2}, {}", offset as i32);
            },
            //bne 3'b001
            0x01 => {
                if rs1_data != rs2_data { 
                    self.pc = self.pc.wrapping_add(offset); 
                    println!("bne pc: {:x}", self.pc);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
                println!("bne x{rs1}, x{rs2}, {}", offset as i32);
            },
            //blt 3'b100
            0x04 => {
                if (rs1_data as i32) < (rs2_data as i32) { 
                    self.pc = self.pc.wrapping_add(offset);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
                println!("blt x{rs1}, x{rs2}, {}", offset as i32);
            },
            //bge 3'b101
            0x05 => {
                if (rs1_data as i32) >= (rs2_data as i32) { 
                    self.pc = self.pc.wrapping_add(offset);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
                println!("bge x{rs1}, x{rs2}, {}", offset as i32);
            },
            //bltu 3'b110
            0x06 => {
                if rs1_data < rs2_data { 
                    self.pc = self.pc.wrapping_add(offset);
                } else {
                    self.pc = self.pc.wrapping_add(4); 
                }
                println!("bltu x{rs1}, x{rs2}, {}", offset as i32);
            },
            //bgeu 3'b111
            0x07 => {
                if rs1_data >= rs2_data { 
                    self.pc = self.pc.wrapping_add(offset);
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
                println!("bgeu x{rs1}, x{rs2}, {}", offset as i32);
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
                let rd = self.set_rd(instr, rs1_data.wrapping_add(s_imm));
                println!("addi x{rd}, x{rs1}, {s_imm}");
            },
            //slti 3'b010
            0x02 => {
                let rd_data = if (rs1_data as i32) < (s_imm as i32) {1} else {0};
                let rd = self.set_rd(instr, rd_data);
                println!("slti x{rd}, x{rs1}, {s_imm}");
            },
            //sltiu 3'b011
            0x03 => {
                let rd_data = if rs1_data < s_imm {1} else {0};
                let rd = self.set_rd(instr, rd_data);
                println!("sltiu x{rd}, x{rs1}, {s_imm}");
            },
            //xori 3'b100
            0x04 => {
                let rd = self.set_rd(instr, rs1_data ^ s_imm);
                println!("xori x{rd}, x{rs1}, {s_imm}");
            },
            //ori 3'b110
            0x06 => {
                let rd = self.set_rd(instr, rs1_data | s_imm);
                println!("ori x{rd}, x{rs1}, {s_imm}");
            },
            //andi 3'b111
            0x07 => {
                let rd = self.set_rd(instr, rs1_data & s_imm);
                println!("andi x{rd}, x{rs1}, {s_imm}");
            },
            //slli 3'b001
            0x01 => {
                let rd = self.set_rd(instr, rs1_data << (s_imm & 0x1f));
                println!("slli x{rd}, x{rs1}, {s_imm}");
            },
            //srli srai 3'b101
            0x05 => {
                match instr>>25 & 0x7f {
                    //srli 7'b000_0000
                    0x00 => {
                        let rd = self.set_rd(instr, rs1_data >> (s_imm & 0x1f));
                        println!("srli x{rd}, x{rs1}, {s_imm}");
                    },
                    //srai 7'b010_0000
                    0x20 => {
                        let rd_data = ((rs1_data as i32) >> (s_imm & 0x1f)) as u32;
                        let rd = self.set_rd(instr, rd_data);
                        println!("srai x{rd}, x{rs1}, {s_imm}");
                    }
                    _ => {
                        println!("math i sr illegal instruction. {:08x}.", instr)
                    },
                }
            },
            //others
            _ => println!("math i illegal instruction. {:08x}.", instr),
        }
    }

    fn execute_math(&mut self, instr: u32) {
        let (rs1, rs1_data) = self.get_rs_1(instr);
        let (rs2, rs2_data) = self.get_rs_2(instr);

        match (instr>>12 & 0x07, instr>>25 & 0x7f) {
            //add sub mul 3'b000
            (0x00, 0x00) => {
                let rd = self.set_rd(instr, rs1_data.wrapping_add(rs2_data));
                println!("add x{rd}, x{rs1}, x{rs2}");
            },
            (0x00, 0x20) => {
                let rd = self.set_rd(instr, rs1_data.wrapping_sub(rs2_data));
                println!("sub x{rd}, x{rs1}, x{rs2}");
            },
            (0x00, 0x01) => {
                let rd_data = rs1_data.wrapping_mul(rs2_data);
                let rd = self.set_rd(instr, rd_data);
                println!("mul x{rd}, x{rs1}, x{rs2}");
            },
            //sll mulh 3'b001
            (0x01, 0x00) => {
                let rd = self.set_rd(instr, rs1_data << (rs2_data & 0x1f));
                println!("sll x{rd}, x{rs1}, x{rs2}");
            },
            (0x01, 0x01) => {
                let rd_data: u32 = (((rs1_data as i32 as i64) * (rs2_data as i32 as i64)) >> 32) as u32;
                let rd = self.set_rd(instr, rd_data);
                println!("mulh x{rd}, x{rs1}, x{rs2}");
            },
            //slt mulhsu 3'b010
            (0x02, 0x00) => {
                let rd_data = if (rs1_data as i32) < (rs2_data as i32) {1} else {0};
                let rd = self.set_rd(instr, rd_data);
                println!("slt x{rd}, x{rs1}, x{rs2}");
            },
            (0x02, 0x01) => {
                let rd_data = (((rs1_data as i32 as i64) * (rs2_data as i64)) >> 32) as u32;
                let rd = self.set_rd(instr, rd_data);
                println!("mulhsu x{rd}, x{rs1}, x{rs2}");
            },
            //sltu mulhu 3'b011
            (0x03, 0x00) => {
                let rd_data = if rs1_data < rs2_data {1} else {0};
                let rd = self.set_rd(instr, rd_data);
                println!("sltu x{rd}, x{rs1}, x{rs2}");
            },
            (0x03, 0x01) => {
                let rd_data = (((rs1_data as u64) * (rs2_data as u64)) >> 32) as u32;
                let rd = self.set_rd(instr, rd_data);
                println!("mulhu x{rd}, x{rs1}, x{rs2}");
            },
            //xor div 3'b100
            (0x04, 0x00) => {
                let rd = self.set_rd(instr, rs1_data ^ rs2_data);
                println!("xor x{rd}, x{rs1}, x{rs2}");
            },
            (0x04, 0x01) => {
                let rd_data: u32 = if rs2_data != 0 {((rs1_data as i32) / (rs2_data as i32)) as u32} else {0xffffffff};
                let rd = self.set_rd(instr, rd_data);
                println!("div x{rd}, x{rs1}, x{rs2}");
            },
            //or rem 3'b110
            (0x06, 0x00) => {
                let rd = self.set_rd(instr, rs1_data | rs2_data);
                println!("or x{rd}, x{rs1}, x{rs2}");
            },
            (0x06, 0x01) => {
                let rd_data: u32 = if rs2_data != 0 {((rs1_data as i32) % (rs2_data as i32)) as u32} else {rs1_data};
                let rd = self.set_rd(instr, rd_data);
                println!("rem x{rd}, x{rs1}, x{rs2}");
            }
            //and remu 3'b111
            (0x07, 0x00) => {
                let rd = self.set_rd(instr, rs1_data & rs2_data);
                println!("and x{rd}, x{rs1}, x{rs2}");
            },
            (0x07, 0x01) => {
                let rd_data: u32 = if rs2_data!= 0 {rs1_data % rs2_data} else {rs1_data};
                let rd = self.set_rd(instr, rd_data);
                println!("remu x{rd}, x{rs1}, x{rs2}");
            },
            //srl sra divu 3'b101
            (0x05, 0x00) => {
                let rd = self.set_rd(instr, rs1_data >> (rs2_data & 0x1f));
                println!("srl x{rd}, x{rs1}, x{rs2}");
            },
            (0x05, 0x20) => {
                let rd_data = ((rs1_data as i32) >> (rs2_data & 0x1f)) as u32;
                let rd = self.set_rd(instr, rd_data);
                println!("sra x{rd}, x{rs1}, x{rs2}");
            },
            (0x05, 0x01) => {
                let rd_data: u32 = if rs2_data!= 0 {rs1_data / rs2_data} else {0xffffffff};
                let rd = self.set_rd(instr, rd_data);
                println!("divu x{rd}, x{rs1}, x{rs2}");
            },
            //others
            _ => println!("math i illegal instruction. {:08x}.", instr),
        }
    }

    fn execute_load(&mut self, instr: u32) {
        let (rs1, rs1_data) = self.get_rs_1(instr);
        let imm = (instr>>20) & 0x00000fff;
        let s_imm = if instr & 0x80000000 == 0x80000000 { 0xfffff000 | imm } else { imm };
        let r_addr = rs1_data.wrapping_add(s_imm);

        match instr>>12 & 0x07 {
            //lb 3'b000
            0x00 => {
                let mut rd_data = 0;
                // self.mems.iter().for_each(|m| {
                //     if m.in_range(r_addr) {
                //         rd_data = m.read_u8(r_addr) as i8 as i32;
                // }});
                for m in self.mems.iter() {
                    if m.in_range(r_addr) {
                        rd_data = m.read_u8(r_addr) as i8 as i32;
                        break;
                    }
                }
                let rd = self.set_rd(instr, rd_data as u32);
                println!("lb x{rd}, {}(x{rs1})", s_imm as i32);
            },
            //lbu 3'b100
            0x04 => {
                let mut rd_data = 0;
                for m in self.mems.iter() {
                    if m.in_range(r_addr) {
                        rd_data = m.read_u8(r_addr);
                        break;
                    }
                }
                let rd = self.set_rd(instr, rd_data as u32);
                println!("lbu x{rd}, {}(x{rs1})", s_imm as i32);
            },
            //lh 3'b001
            0x01 => {
                let mut rd_data = 0;
                for m in self.mems.iter() {
                    if m.in_range(r_addr) {
                        rd_data = m.read_u16(r_addr) as i16 as i32;
                        break;
                    }
                }
                let rd = self.set_rd(instr, rd_data as u32);
                println!("lh x{rd}, {}(x{rs1})", s_imm as i32);
            },
            //lhu 3'b101
            0x05 => {
                let mut rd_data = 0;
                for m in self.mems.iter() {
                    if m.in_range(r_addr) {
                        rd_data = m.read_u16(r_addr);
                        break;
                    }
                }
                let rd = self.set_rd(instr, rd_data as u32);
                println!("lhu x{rd}, {}(x{rs1})", s_imm as i32);
            },
            //lw 3'b010
            0x02 => {
                let mut rd_data = 0 ;
                for m in self.mems.iter() {
                    if m.in_range(r_addr) {
                        rd_data = m.read_u32(r_addr);
                        break;
                    }
                }
                for p in self.perips.iter() {
                    if p.in_range(r_addr) {
                        rd_data = p.read_u32(r_addr);
                        break;
                    }
                }
                let rd = self.set_rd(instr, rd_data);
                println!("lw x{rd}, {}(x{rs1})", s_imm as i32);
            },
            //others
            _ => println!("load illegal instruction. {:08x}.", instr),
        }
    }

    fn execute_store(&mut self, instr: u32) {
        let (rs1, rs1_data) = self.get_rs_1(instr);
        let (rs2, rs2_data) = self.get_rs_2(instr);
        let imm = ((instr>>20) & 0x000007e0) | ((instr>>7) & 0x0000001f);
        let s_imm = if instr & 0x80000000 == 0x80000000 { 0xfffff800 | imm } else { imm };
        let wr_addr = rs1_data.wrapping_add(s_imm);

        match instr>>12 & 0x07 {
            //sb 3'b000
            0x00 => {
                for m in self.mems.iter_mut() {
                    if m.in_range(wr_addr) {
                        m.write_u8((rs2_data & 0xff) as u8, wr_addr);
                        break;
                    }
                }
                println!("sb x{rs2}, {}(x{rs1})", s_imm as i32);
            },
            //sh 3'b001
            0x01 => {
                for m in self.mems.iter_mut() {
                    if m.in_range(wr_addr) {
                        m.write_u16((rs2_data & 0xffff) as u16, wr_addr);
                        break;
                    }
                }
                println!("sh x{rs2}, {}(x{rs1})", s_imm as i32);
            },
            //sw 3'b010
            0x02 => {
                for m in self.mems.iter_mut() {
                    if m.in_range(wr_addr) {
                        m.write_u32(rs2_data, wr_addr);
                        break;
                    }
                }
                for p in self.perips.iter_mut() {
                    if p.in_range(wr_addr) {
                        p.write_u32(rs2_data, wr_addr);
                        break;
                    }
                }
                println!("sb x{rs2}, {}(x{rs1})", s_imm as i32);
            },
            //others
            _ => println!("store illegal instruction. {:08x}.", instr),
        }
    }

    fn execute_fence(&mut self, instr: u32) {
        match instr>>12 & 0x07 {
            //fence 3'b000
            0x00 => {
                println!("fence {}, {}", (instr>>24)&0x0f, (instr>>20)&0x0f);
            },
            //fence.i 3'b001
            0x01 => {
                println!("fence.i");
            },
            _ => panic!("pc:{:x} fence illegal instruction. {:08x}.", self.pc, instr),
        }
    }

    fn execute_sys(&mut self, instr: u32) {
        let (rs1, rs1_data) = self.get_rs_1(instr);
        let csr = instr>>20 & 0xfff;

        match (instr>>12 & 0x07, instr>>20 & 0xfff) {
            //ecall 3'b000, 12'h0
            (0x00, 0x000) => {
                println!("ecall");
            },
            //ebreak 3'b000, 12'h1
            (0x00, 0x001) => {
                println!("ebreak");
            },
            //mret 3'b000, 12'h1
            (0x00, 0x302) => {
                println!("mret");
            },
            //csrrw 3'b001, *
            (0x01, _) => {
                let t = self.csr.read(csr);
                self.csr.write(csr, rs1_data);
                let rd = self.set_rd(instr, t);
                println!("csrrw {rd}, {csr}, {rs1}");
            },
            //csrrs 3'b010, *
            (0x02, _) => {
                let t = self.csr.read(csr);
                self.csr.write(csr, rs1_data | t);
                let rd = self.set_rd(instr, t);
                println!("csrrs {rd}, {csr}, {rs1}");
            },
            //csrrc 3'b011, *
            (0x03, _) => {
                let t = self.csr.read(csr);
                self.csr.write(csr, (!rs1_data) & t);
                let rd = self.set_rd(instr, t);
                println!("csrrc {rd}, {csr}, {rs1}");
            },
            //csrrwi 3'b101, *
            (0x05, _) => {
                let t = self.csr.read(csr);
                self.csr.write(csr, rs1 as u32);
                let rd = self.set_rd(instr, t);
                println!("csrrwi {rd}, {csr}, {rs1}");
            },
            //csrrsi 3'b110, *
            (0x06, _) => {
                let t = self.csr.read(csr);
                self.csr.write(csr, (rs1 as u32) | t);
                let rd = self.set_rd(instr, t);
                println!("csrrsi {rd}, {csr}, {rs1}");
            },
            //csrrci 3'b111, *
            (0x07, _) => {
                let t = self.csr.read(csr);
                self.csr.write(csr, (!(rs1 as u32)) & t);
                let rd = self.set_rd(instr, t);
                println!("csrrci {rd}, {csr}, {rs1}");
            },
            _ => panic!("pc:{:x} sys illegal instruction. {:08x}.", self.pc, instr),
        }
    }

    pub fn print_mem(&self, addr: u32) {
        let addr2 = addr & 0xffffff00;
        for m in self.mems.iter() {
            if m.in_range(addr2) {
                println!("{}", m.dump(addr2));
                break;
            }
        }
    }

    pub fn print_reg(&self) {
        println!("{}", self.reg.to_string());
    }

    pub fn print_csr(&self) {
        println!("{}", self.csr.to_string());
    }

    pub fn print_perips(&self, name: &String) {
        for p in self.perips.iter() {
            if p.name().eq(name) {
                println!("{}", p.dump(0));
            }
        }
    }
}