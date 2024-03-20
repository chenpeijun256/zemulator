use std::boxed::Box;
use std::fmt::Debug;

pub trait MemIO {
    fn in_range(&self, addr: u32) -> bool;
    fn read_u32(&self, addr: u32) -> u32;
    fn write_u32(&mut self, data: u32, addr: u32);
    fn dump(&self, addr: u32) -> String;
}

pub struct Mem {
    data: Box<[u8]>,
    start: u32,
    size: u32,
    name: String,
}

impl Debug for Mem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mem")
            .field("start", &self.start)
            .field("size", &self.size)
            .field("name", &self.name)
            .finish()
    }
}

impl MemIO for Mem {
    fn in_range(&self, addr: u32) -> bool {
        addr >= self.start && addr < self.start + self.size
    }

    fn write_u32(&mut self, data: u32, addr: u32) {
        let pos = (addr - self.start) as usize;
        self.data[pos] = data as u8;
        self.data[pos+1] = (data>>8) as u8;
        self.data[pos+2] = (data>>16) as u8;
        self.data[pos+3] = (data>>24) as u8;
    }

    fn read_u32(&self, addr: u32) -> u32 {
        let pos = (addr - self.start) as usize;
        ((self.data[pos+3] as u32) << 24) | 
        ((self.data[pos+2] as u32) << 16) | 
        ((self.data[pos+1] as u32) << 8) | 
        (self.data[pos] as u32)
    }

    fn dump(&self, addr: u32) -> String {
        let pos = (addr - self.start) as usize;
        let mut res = String::new();
        for i in 0..128 {
            if i % 16 == 0 {
                res.push_str(&format!("\n{:08X }: ", pos + i));
            }
            res.push_str(&format!("{:02X} ", self.data[pos + i]));
        }
        return res;
    }
}

impl Mem {
    // pub fn new_with_data(name: String, start: u32, size: u32, data: Vec<u8>) -> Self {
    //     Mem { data: data.into_boxed_slice(), name, start, size }
    // }

    pub fn new(name: String, start: u32, size: u32) -> Self {
        Mem { data: vec![0; size as usize].into_boxed_slice(), 
                name, start, size }
    }

    pub fn fill(&mut self, data: Vec<u8>, addr: u32) {
        if addr >= self.start && addr + (data.len() as u32) < self.start + self.size {
            let pos = (addr - self.start) as usize;
            for (i, &elem) in data.iter().enumerate() {
                self.data[pos + i] = elem;
            }
        }
    }

    pub fn match_name(&self, name: &String) -> bool {
        self.name.eq(name)
    }

    // pub fn length(&self) -> usize {
    //     self.data.len()
    // }

    pub fn write_u8(&mut self, data: u8, addr: u32) {
        let pos = (addr - self.start) as usize;
        self.data[pos] = data;
    }

    pub fn read_u8(&self, addr: u32) -> u8 {
        let pos = (addr - self.start) as usize;
        self.data[pos]
    }

    pub fn write_u16(&mut self, data: u16, addr: u32) {
        let pos = (addr - self.start) as usize;
        self.data[pos] = data as u8;
        self.data[pos+1] = (data>>8) as u8;
    }

    pub fn read_u16(&self, addr: u32) -> u16 {
        let pos = (addr - self.start) as usize;
        ((self.data[pos+1] as u16) << 8) | (self.data[pos] as u16)
    }
}