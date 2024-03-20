use crate::mem::MemIO;


#[derive(Debug)]
pub struct Perips {
    name: String,
    registers: Vec<u32>,

    start: u32,
    size: u32,

    intr: u32,
}

impl MemIO for Perips {
    fn in_range(&self, addr: u32) -> bool {
        addr >= self.start && addr < self.start + self.size * 4
    }

    fn read_u32(&self, addr: u32) -> u32 {
        if self.in_range(addr) {
            return self.registers[((addr - self.start)>>2) as usize];
        } else {
            println!("{addr} is not exist in this perips.");
            return 0;
        }
    }

    fn write_u32(&mut self, data: u32, addr: u32) {
        if self.in_range(addr) {
            self.registers[((addr - self.start)>>2) as usize] = data;
        } else {
            println!("{addr} is not exist in this perips.");
        }
    }

    fn dump(&self, _addr: u32) -> String {
        let mut res = String::new();
        res.push_str(&format!("{}: {:08X}+{}, intr:{:08X}\n", self.name, self.start, self.size, self.intr));
        for i in self.registers.iter() {
            res.push_str(&format!("{:08X} ", i));
        }
        return res;
    }
}

impl Perips {

    pub fn new(name: String, start: u32, size: u32, intr: u32) -> Self {
        Perips {name,
                registers: vec![0; size as usize], 
                start, 
                size,
                intr,
            }
    }

    // pub fn name(&self) -> &String {
    //     &self.name
    // }

    pub fn match_name(&self, name: &String) -> bool {
        self.name.eq(name)
    }

    pub fn get_intr(&self) -> u32 {
        self.registers[((self.intr - self.start)>>2) as usize]
    }

    pub fn clear_intr(&mut self) {
        self.registers[((self.intr - self.start)>>2) as usize] = 0;
    }
}
