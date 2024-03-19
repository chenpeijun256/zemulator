mod com_reg;
mod csr_reg;
pub mod cpu;

use crate::rv32_actor::cpu::Rv32Cpu;
use crate::perips::Perips;
use crate::mem::{Mem, MemIO};

const REG_NAME:[&str; 32] = ["zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", 
                "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
                "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
                "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"];

pub struct Rv32Actor {
    cpus: Vec<Rv32Cpu>,

    mems: Vec<Mem>,
    perips: Vec<Perips>,
}

impl Rv32Actor {
    pub fn new() -> Self {
        Rv32Actor{
                    cpus: Vec::new(),
                    mems: Vec::new(),
                    perips: Vec::new(),
                }
    }

    pub fn add_cpu(&mut self, cpu: Rv32Cpu) {
        self.cpus.push(cpu);
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

    pub fn tick(&mut self) {
        for cpu in self.cpus.iter() {
            let pc = cpu.get_pc();
            let instr = self.mems[0].read_u32(pc);
            println!("(){}, pc: {:x}, instr: {:08x}", cpu.get_time(), pc, instr);
            self.execute(cpu, pc, instr);
        }
    }

    fn execute(&self, cpu: &Rv32Cpu, pc: u32, instr: u32) {
        //opcode = instr[6:0];
        match instr & 0x7f {
            //lui 7'b0110111
            0x37 => {
                Rv32Actor::execute_lui(cpu, instr);
                cpu.set_pc(pc.wrapping_add(4));
            },
            //auipc 7'b0010111
            0x17 => {
                Rv32Actor::execute_auipc(cpu, pc, instr);
                cpu.set_pc(pc.wrapping_add(4));
            },
            //jal 7'b1101111
            0x6f => {
                Rv32Actor::execute_jal(cpu, pc, instr);
            },
            //jalr = (opcode == 7'b1100111);
            0x67 => {
                Rv32Actor::execute_jalr(cpu, pc, instr);
            },
            //jb  7'b1100011
            0x63 => {
                Rv32Actor::execute_jb(cpu, pc, instr);
            },
            //math i 7'b0010011
            0x13 => {
                Rv32Actor::execute_math_i(cpu, instr);
                cpu.set_pc(pc.wrapping_add(4));
            },
            //math 7'b0110011
            0x33 => {
                Rv32Actor::execute_math(cpu, instr);
                cpu.set_pc(pc.wrapping_add(4));
            },
            //load, 7'b0000011
            0x03 => {
                self.execute_load(cpu, instr);
                cpu.set_pc(pc.wrapping_add(4));
            },
            //store, 7'b0100011
            0x23 => {
                self.execute_store(cpu, instr);
                cpu.set_pc(pc.wrapping_add(4));
            },
            //fence  7'b0001111
            0x0f => {
                Rv32Actor::execute_fence(cpu, instr);
                cpu.set_pc(pc.wrapping_add(4));
            },
            //sys 7'b1110011
            0x73 => {
                Rv32Actor::execute_sys(cpu, instr);
                cpu.set_pc(pc.wrapping_add(4));
            },
            //others
            _ => {
                panic!("pc: {:x} inllegal instruction. {:08x}.", pc, instr);
            },
        }
    }

    // pub fn get_rs(&self, index: u32) -> u32 {
    //     self.reg.read(index)
    // }

    // fn set_rd(&mut self, instr: u32, data: u32) -> u32 {
    //     let rd = instr>>7 & 0x1f;
    //     if rd != 0 {
    //         self.reg.write(rd, data);
    //     }
    //     rd
    // }

    // fn get_rs_1(&self, instr: u32) -> (u32, u32) {
    //     let r1 = instr>>15 & 0x1f;
    //     if r1 == 0 {
    //         return (0, 0);
    //     }
    //     (r1, self.reg.read(r1))
    // }

    // fn get_rs_2(&mut self, instr: u32) -> (u32, u32) {
    //     let r2 = instr>>20 & 0x1f;
    //     if r2 == 0 {
    //         return (0, 0);
    //     }
    //     (r2, self.reg.read(r2))
    // }

    fn execute_lui(cpu: &Rv32Cpu, instr: u32) {
        let imm = instr & 0xfffff000;
        let rd = cpu.set_rd(instr, imm);
        println!("lui {}, {:x}", REG_NAME[rd], imm);
    }

    fn execute_auipc(cpu: &Rv32Cpu, pc: u32, instr: u32) {
        let imm = instr & 0xfffff000;
        let rd = cpu.set_rd(instr, imm + pc);
        println!("auipc {}, {:x}", REG_NAME[rd], imm);
    }

    fn execute_jal(cpu: &Rv32Cpu, pc: u32, instr: u32) {
        let imm = (instr & 0x000ff000) | 
                    ((instr>>8) & 0x00000800) | 
                    ((instr>>20) & 0x000007fe);
        let offset = if instr & 0x80000000 == 0x80000000 {0xfff00000 | imm } else { imm };
        let rd = cpu.set_rd(instr, pc + 4);
        cpu.set_pc(pc.wrapping_add(offset));
        println!("jal {}, {}", REG_NAME[rd], offset as i32);
    }

    fn execute_jalr(cpu: &Rv32Cpu, pc: u32, instr: u32) {
        let imm = (instr>>20) & 0x00000fff;
        let offset = if instr & 0x80000000 == 0x80000000 { 0xfffff000 | imm } else { imm };
        let rd = cpu.set_rd(instr, pc + 4);
        let (rs1, r1_data) = cpu.get_rs_1(instr);
        cpu.set_pc((r1_data.wrapping_add(offset)) & 0xfffffffe);
        println!("jalr {}, {}({})", REG_NAME[rd], offset as i32, REG_NAME[rs1]);
    }

    fn execute_jb(cpu: &Rv32Cpu, pc: u32, instr: u32) {
        let (rs1, rs1_data) = cpu.get_rs_1(instr);
        let (rs2, rs2_data) = cpu.get_rs_2(instr);
        let imm = ((instr<<4) & 0x00000800) | 
                    ((instr>>20) & 0x000007e0) | 
                    ((instr>>7) & 0x0000001e);
        let offset = if instr & 0x80000000 == 0x80000000 {0xfffff000 | imm } else { imm };
        match instr>>12 & 0x07 {
            //beq 3'b000
            0x00 => {
                if rs1_data == rs2_data { 
                    cpu.set_pc(pc.wrapping_add(offset)); 
                } else {
                    cpu.set_pc(pc.wrapping_add(4));
                }
                println!("beq {}, {}, {}", REG_NAME[rs1], REG_NAME[rs2], offset as i32);
            },
            //bne 3'b001
            0x01 => {
                if rs1_data != rs2_data { 
                    cpu.set_pc(pc.wrapping_add(offset)); 
                    // println!("bne pc: {:x}", pc);
                } else {
                    cpu.set_pc(pc.wrapping_add(4));
                }
                println!("bne {}, {}, {}", REG_NAME[rs1], REG_NAME[rs2], offset as i32);
            },
            //blt 3'b100
            0x04 => {
                if (rs1_data as i32) < (rs2_data as i32) { 
                    cpu.set_pc(pc.wrapping_add(offset));
                } else {
                    cpu.set_pc(pc.wrapping_add(4));
                }
                println!("blt {}, {}, {}", REG_NAME[rs1], REG_NAME[rs2], offset as i32);
            },
            //bge 3'b101
            0x05 => {
                if (rs1_data as i32) >= (rs2_data as i32) { 
                    cpu.set_pc(pc.wrapping_add(offset));
                } else {
                    cpu.set_pc(pc.wrapping_add(4));
                }
                println!("bge {}, {}, {}", REG_NAME[rs1], REG_NAME[rs2], offset as i32);
            },
            //bltu 3'b110
            0x06 => {
                if rs1_data < rs2_data { 
                    cpu.set_pc(pc.wrapping_add(offset));
                } else {
                    cpu.set_pc(pc.wrapping_add(4)); 
                }
                println!("bltu {}, {}, {}", REG_NAME[rs1], REG_NAME[rs2], offset as i32);
            },
            //bgeu 3'b111
            0x07 => {
                if rs1_data >= rs2_data { 
                    cpu.set_pc(pc.wrapping_add(offset));
                } else {
                    cpu.set_pc(pc.wrapping_add(4));
                }
                println!("bgeu {}, {}, {}", REG_NAME[rs1], REG_NAME[rs2], offset as i32);
            },
            //others
            _ => println!("jb illegal instruction. {:08x}.", instr),
        }
    }

    fn execute_math_i(cpu: &Rv32Cpu, instr: u32) {
        let (rs1, rs1_data) = cpu.get_rs_1(instr);
        let imm = (instr>>20) & 0x00000fff;
        let s_imm = if instr & 0x80000000 == 0x80000000 { 0xfffff000 | imm } else { imm };
        // let s_imm = e_imm as i32;

        match instr>>12 & 0x07 {
            //addi 3'b000
            0x00 => {
                let rd = cpu.set_rd(instr, rs1_data.wrapping_add(s_imm));
                println!("addi {}, {}, {s_imm}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //slti 3'b010
            0x02 => {
                let rd_data = if (rs1_data as i32) < (s_imm as i32) {1} else {0};
                let rd = cpu.set_rd(instr, rd_data);
                println!("slti {}, {}, {s_imm}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //sltiu 3'b011
            0x03 => {
                let rd_data = if rs1_data < s_imm {1} else {0};
                let rd = cpu.set_rd(instr, rd_data);
                println!("sltiu {}, {}, {s_imm}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //xori 3'b100
            0x04 => {
                let rd = cpu.set_rd(instr, rs1_data ^ s_imm);
                println!("xori {}, {}, {s_imm}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //ori 3'b110
            0x06 => {
                let rd = cpu.set_rd(instr, rs1_data | s_imm);
                println!("ori {}, {}, {s_imm}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //andi 3'b111
            0x07 => {
                let rd = cpu.set_rd(instr, rs1_data & s_imm);
                println!("andi {}, {}, {s_imm}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //slli 3'b001
            0x01 => {
                let rd = cpu.set_rd(instr, rs1_data << (s_imm & 0x1f));
                println!("slli {}, {}, {s_imm}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //srli srai 3'b101
            0x05 => {
                match instr>>25 & 0x7f {
                    //srli 7'b000_0000
                    0x00 => {
                        let rd = cpu.set_rd(instr, rs1_data >> (s_imm & 0x1f));
                        println!("srli {}, {}, {s_imm}", REG_NAME[rd], REG_NAME[rs1]);
                    },
                    //srai 7'b010_0000
                    0x20 => {
                        let rd_data = ((rs1_data as i32) >> (s_imm & 0x1f)) as u32;
                        let rd = cpu.set_rd(instr, rd_data);
                        println!("srai {}, {}, {s_imm}", REG_NAME[rd], REG_NAME[rs1]);
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

    fn execute_math(cpu: &Rv32Cpu, instr: u32) {
        let (rs1, rs1_data) = cpu.get_rs_1(instr);
        let (rs2, rs2_data) = cpu.get_rs_2(instr);

        match (instr>>12 & 0x07, instr>>25 & 0x7f) {
            //add sub mul 3'b000
            (0x00, 0x00) => {
                let rd = cpu.set_rd(instr, rs1_data.wrapping_add(rs2_data));
                println!("add {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            (0x00, 0x20) => {
                let rd = cpu.set_rd(instr, rs1_data.wrapping_sub(rs2_data));
                println!("sub {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            (0x00, 0x01) => {
                let rd_data = rs1_data.wrapping_mul(rs2_data);
                let rd = cpu.set_rd(instr, rd_data);
                println!("mul {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            //sll mulh 3'b001
            (0x01, 0x00) => {
                let rd = cpu.set_rd(instr, rs1_data << (rs2_data & 0x1f));
                println!("sll {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            (0x01, 0x01) => {
                let rd_data: u32 = (((rs1_data as i32 as i64) * (rs2_data as i32 as i64)) >> 32) as u32;
                let rd = cpu.set_rd(instr, rd_data);
                println!("mulh {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            //slt mulhsu 3'b010
            (0x02, 0x00) => {
                let rd_data = if (rs1_data as i32) < (rs2_data as i32) {1} else {0};
                let rd = cpu.set_rd(instr, rd_data);
                println!("slt {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            (0x02, 0x01) => {
                let rd_data = (((rs1_data as i32 as i64) * (rs2_data as i64)) >> 32) as u32;
                let rd = cpu.set_rd(instr, rd_data);
                println!("mulhsu {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            //sltu mulhu 3'b011
            (0x03, 0x00) => {
                let rd_data = if rs1_data < rs2_data {1} else {0};
                let rd = cpu.set_rd(instr, rd_data);
                println!("sltu {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            (0x03, 0x01) => {
                let rd_data = (((rs1_data as u64) * (rs2_data as u64)) >> 32) as u32;
                let rd = cpu.set_rd(instr, rd_data);
                println!("mulhu {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            //xor div 3'b100
            (0x04, 0x00) => {
                let rd = cpu.set_rd(instr, rs1_data ^ rs2_data);
                println!("xor {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            (0x04, 0x01) => {
                let rd_data: u32 = if rs2_data != 0 {((rs1_data as i32) / (rs2_data as i32)) as u32} else {0xffffffff};
                let rd = cpu.set_rd(instr, rd_data);
                println!("div {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            //or rem 3'b110
            (0x06, 0x00) => {
                let rd = cpu.set_rd(instr, rs1_data | rs2_data);
                println!("or {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            (0x06, 0x01) => {
                let rd_data: u32 = if rs2_data != 0 {((rs1_data as i32) % (rs2_data as i32)) as u32} else {rs1_data};
                let rd = cpu.set_rd(instr, rd_data);
                println!("rem {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            }
            //and remu 3'b111
            (0x07, 0x00) => {
                let rd = cpu.set_rd(instr, rs1_data & rs2_data);
                println!("and {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            (0x07, 0x01) => {
                let rd_data: u32 = if rs2_data!= 0 {rs1_data % rs2_data} else {rs1_data};
                let rd = cpu.set_rd(instr, rd_data);
                println!("remu {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            //srl sra divu 3'b101
            (0x05, 0x00) => {
                let rd = cpu.set_rd(instr, rs1_data >> (rs2_data & 0x1f));
                println!("srl {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            (0x05, 0x20) => {
                let rd_data = ((rs1_data as i32) >> (rs2_data & 0x1f)) as u32;
                let rd = cpu.set_rd(instr, rd_data);
                println!("sra {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            (0x05, 0x01) => {
                let rd_data: u32 = if rs2_data!= 0 {rs1_data / rs2_data} else {0xffffffff};
                let rd = cpu.set_rd(instr, rd_data);
                println!("divu {}, {}, {}", REG_NAME[rd], REG_NAME[rs1], REG_NAME[rs2]);
            },
            //others
            _ => println!("math i illegal instruction. {:08x}.", instr),
        }
    }

    fn execute_load(&self, cpu: &Rv32Cpu, instr: u32) {
        let (rs1, rs1_data) = cpu.get_rs_1(instr);
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
                let rd = cpu.set_rd(instr, rd_data as u32);
                println!("lb x{rd}, {}({})", s_imm as i32, REG_NAME[rs1]);
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
                let rd = cpu.set_rd(instr, rd_data as u32);
                println!("lbu x{rd}, {}({})", s_imm as i32, REG_NAME[rs1]);
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
                let rd = cpu.set_rd(instr, rd_data as u32);
                println!("lh x{rd}, {}({})", s_imm as i32, REG_NAME[rs1]);
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
                let rd = cpu.set_rd(instr, rd_data as u32);
                println!("lhu x{rd}, {}({})", s_imm as i32, REG_NAME[rs1]);
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
                let rd = cpu.set_rd(instr, rd_data);
                println!("lw x{rd}, {}({})", s_imm as i32, REG_NAME[rs1]);
            },
            //others
            _ => println!("load illegal instruction. {:08x}.", instr),
        }
    }

    fn execute_store(&self, cpu: &Rv32Cpu, instr: u32) {
        let (rs1, rs1_data) = cpu.get_rs_1(instr);
        let (rs2, rs2_data) = cpu.get_rs_2(instr);
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
                println!("sb {}, {}({})", REG_NAME[rs2], s_imm as i32, REG_NAME[rs1]);
            },
            //sh 3'b001
            0x01 => {
                for m in self.mems.iter_mut() {
                    if m.in_range(wr_addr) {
                        m.write_u16((rs2_data & 0xffff) as u16, wr_addr);
                        break;
                    }
                }
                println!("sh {}, {}({})", REG_NAME[rs2], s_imm as i32, REG_NAME[rs1]);
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
                println!("sb {}, {}({})", REG_NAME[rs2], s_imm as i32, REG_NAME[rs1]);
            },
            //others
            _ => println!("store illegal instruction. {:08x}.", instr),
        }
    }

    fn execute_fence(cpu: &Rv32Cpu, instr: u32) {
        match instr>>12 & 0x07 {
            //fence 3'b000
            0x00 => {
                println!("fence {}, {}", (instr>>24)&0x0f, (instr>>20)&0x0f);
            },
            //fence.i 3'b001
            0x01 => {
                println!("fence.i");
            },
            _ => panic!("fence illegal instruction. {:08x}.", instr),
        }
    }

    fn execute_sys(cpu: &Rv32Cpu, instr: u32) {
        let (rs1, rs1_data) = cpu.get_rs_1(instr);
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
                let t = cpu.read_csr(csr);
                cpu.write_csr(csr, rs1_data);
                let rd = cpu.set_rd(instr, t);
                println!("csrrw {}, {csr}, {}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //csrrs 3'b010, *
            (0x02, _) => {
                let t = cpu.read_csr(csr);
                cpu.write_csr(csr, rs1_data | t);
                let rd: usize = cpu.set_rd(instr, t);
                println!("csrrs {}, {csr}, {}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //csrrc 3'b011, *
            (0x03, _) => {
                let t = cpu.read_csr(csr);
                cpu.write_csr(csr, (!rs1_data) & t);
                let rd = cpu.set_rd(instr, t);
                println!("csrrc {}, {csr}, {}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //csrrwi 3'b101, *
            (0x05, _) => {
                let t = cpu.read_csr(csr);
                cpu.write_csr(csr, rs1 as u32);
                let rd = cpu.set_rd(instr, t);
                println!("csrrwi {}, {csr}, {}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //csrrsi 3'b110, *
            (0x06, _) => {
                let t = cpu.read_csr(csr);
                cpu.write_csr(csr, (rs1 as u32) | t);
                let rd = cpu.set_rd(instr, t);
                println!("csrrsi {}, {csr}, {}", REG_NAME[rd], REG_NAME[rs1]);
            },
            //csrrci 3'b111, *
            (0x07, _) => {
                let t = cpu.read_csr(csr);
                cpu.write_csr(csr, (!(rs1 as u32)) & t);
                let rd = cpu.set_rd(instr, t);
                println!("csrrci {}, {csr}, {}", REG_NAME[rd], REG_NAME[rs1]);
            },
            _ => panic!("sys illegal instruction. {:08x}.", instr),
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
        self.cpus.iter().for_each(|c| c.print_reg());
    }

    pub fn print_csr(&self) {
        self.cpus.iter().for_each(|c|c.print_csr());
    }

    pub fn print_perips(&self, name: &String) {
        for p in self.perips.iter() {
            if p.name().eq(name) {
                println!("{}", p.dump(0));
            }
        }
    }
}