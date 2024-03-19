use crate::rv32_actor::csr_reg::CsrReg;
use crate::rv32_actor::com_reg::ComReg;

pub struct Rv32Cpu {
    name: String,
    
    tick_cnt: u32,
    freq: f32,

    pc: u32,

    reg: ComReg,
    csr: CsrReg,
}

impl Rv32Cpu {
    pub fn new(name: String, rst_pc: u32, freq: f32) -> Self {
        Rv32Cpu{
                    name,
                    freq,
                    pc: rst_pc, 
                    reg: ComReg::new(32), 
                    tick_cnt: 0, 
                    csr: CsrReg::new(), 
                }
    }

    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn tick(&mut self) {
        self.tick_cnt += 1;
    }

    pub fn get_time(&self) -> f64 {
        (self.tick_cnt as f64) / ((self.freq * 1000.0 * 1000.0) as f64)
    }

    pub fn get_rs(&self, index: u32) -> u32 {
        self.reg.read(index)
    }

    pub fn set_rd(&mut self, instr: u32, data: u32) -> usize {
        let rd = instr>>7 & 0x1f;
        if rd != 0 {
            self.reg.write(rd, data);
        }
        rd as usize
    }

    pub fn get_rs_1(&self, instr: u32) -> (usize, u32) {
        let r1 = instr>>15 & 0x1f;
        if r1 == 0 {
            return (0, 0);
        }
        (r1 as usize, self.reg.read(r1))
    }

    pub fn get_rs_2(&mut self, instr: u32) -> (usize, u32) {
        let r2 = instr>>20 & 0x1f;
        if r2 == 0 {
            return (0, 0);
        }
        (r2 as usize, self.reg.read(r2))
    }

    pub fn read_csr(&self, addr: u32) -> u32 {
        self.csr.read(addr) 
    }

    pub fn write_csr(&self, addr: u32, dat: u32) {
        self.csr.write(addr, dat);
    }

    pub fn print_reg(&self) {
        println!("{}", self.reg.to_string());
    }

    pub fn print_csr(&self) {
        println!("{}", self.csr.to_string());
    }
}

