use crate::intrrupt::IntrType;
use crate::rv32_actor::csr_reg::CsrReg;
use crate::rv32_actor::com_reg::ComReg;

pub struct Rv32Cpu {
    name: String,

    freq: f32,

    pc: u32,
    exception: IntrType,

    reg: ComReg,
    csr: CsrReg,
}

impl Rv32Cpu {
    pub fn new(name: String, rst_pc: u32, freq: f32) -> Self {
        Rv32Cpu{
                    name,
                    freq,
                    pc: rst_pc, 
                    exception: IntrType::None,
                    reg: ComReg::new(32), 
                    csr: CsrReg::new(), 
                }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn match_name(&self, name: &String) -> bool {
        self.name.eq(name)
    }

    pub fn exception(&self) -> IntrType {
        self.exception
    }

    pub fn set_exception(&mut self, exce: IntrType) {
        self.exception = exce;
    }

    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn get_rs(&self, index: u32) -> u32 {
        self.reg.read(index)
    }

    pub fn set_rs(&mut self, index: u32, data: u32) {
        self.reg.write(index, data);
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

    pub fn write_csr(&mut self, addr: u32, dat: u32) {
        self.csr.write(addr, dat);
    }

    pub fn print_reg(&self) {
        println!("{} Reg:\n{}", self.name, self.reg.to_string());
    }

    pub fn print_csr(&self) {
        println!("{} Csr:\n{}", self.name, self.csr.to_string());
    }
}

