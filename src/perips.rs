
#[derive(Debug)]
pub struct Perips {
    name: String,
    registers: Vec<u32>,

    start_addr: u32,
    reg_size: u32,

    intr: u32,
}

impl Perips {

    pub fn new(name: String, addr: u32, size: u32, intr: u32) -> Self {
        Perips {name,
                registers: vec![0; size as usize], 
                start_addr: addr, 
                reg_size: size,
                intr,
            }
    }

    pub fn read(&self, addr: u32) -> u32 {
        if addr < self.start_addr + self.reg_size && addr >= self.start_addr {
            return self.registers[((addr - self.start_addr)>>2) as usize];
        } else {
            println!("{addr} is not exist in perips.");
            return 0;
        }
    }

    pub fn write(&mut self, addr: u32, dat: u32) {
        if addr < self.start_addr + self.reg_size && addr >= self.start_addr {
            self.registers[((addr - self.start_addr)>>2) as usize] = dat;
        } else {
            println!("{addr} is not exist in perips.");
        }
    }
}
