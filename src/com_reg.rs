use std::usize;

pub struct ComReg {
    reg: Vec<u32>,
}

impl ComReg {
    pub fn new(size: usize) ->Self {
        ComReg{reg : vec![0; size]}
    }

    pub fn read(&self, rs: u32) -> u32 {
        self.reg[rs as usize]
    }

    pub fn write(&mut self, rs: u32, dat: u32) {
        self.reg[rs as usize] = dat;
    }
}

impl std::fmt::Display for ComReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg_name = ["zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", 
                                    "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
                                    "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
                                    "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"];
        let mut out_str = "Reg-------------------------\n".to_owned();
        for (i, elem) in self.reg.iter().enumerate() {
            out_str.push_str(&format!("{}={:x}({}), ", reg_name[i], elem, elem));
            if i % 8 == 7 {
                out_str.push_str("\n");
            }
        }
        out_str.push_str("----------------------------");
        write!(f, "{}", out_str)
    }
}
