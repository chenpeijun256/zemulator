
pub struct CsrReg {
    mepc: u32,
    mcause: u32,
    mstatus: u32,
    mtvec: u32,
}

impl CsrReg {
    pub fn new() ->Self {
        CsrReg{mepc: 0, mcause: 0, mstatus: 0, mtvec: 0}
    }

    pub fn read(&self, addr: u32) -> u32 {
        match addr {
            0x341 => self.mepc,
            0x342 => self.mcause,
            0x300 => self.mstatus,
            0x305 => self.mtvec,
            _ => panic!("csr read address {:x} not exist.", addr),
        }
    }

    pub fn write(&mut self, addr: u32, dat: u32) {
        match addr {
            0x341 => self.mepc = dat,
            0x342 => self.mcause = dat,
            0x300 => self.mstatus = dat,
            0x305 => self.mtvec = dat,
            _ => panic!("csr write address {:x} not exist.", addr),
        }
    }
}

impl std::fmt::Display for CsrReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::new();
        out_str.push_str(&format!("mepc={:x}.\n", self.mepc));
        out_str.push_str(&format!("mcause={:x}({:b}).\n", self.mcause, self.mcause));
        out_str.push_str(&format!("mstatus={:x}({:b}).\n", self.mstatus, self.mstatus));
        out_str.push_str(&format!("mtvec={:x}.\n", self.mtvec));
        write!(f, "{}", out_str)
    }
}