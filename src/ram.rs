
pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn new_with_data(data: Vec<u8>) -> Self {
        Ram { data }
    }

    pub fn new(len: usize) -> Self {
        Ram { data: vec![0; len] }
    }

    pub fn fill(&mut self, data: Vec<u8>, pos: usize) {
        let mut i = 0;
        for elem in data {
            self.data[pos + i] = elem;
            i += 1;
        }
    }

    // pub fn length(&self) -> usize {
    //     self.data.len()
    // }

    pub fn write_u8(&mut self, data: u8, pos: usize) {
        self.data[pos] = data;
    }

    pub fn read_u8(&self, pos: usize) -> u8 {
        self.data[pos]
    }

    pub fn write_u16(&mut self, data: u16, pos: usize) {
        self.data[pos] = data as u8;
        self.data[pos+1] = (data>>8) as u8;
    }

    pub fn read_u16(&self, pos: usize) -> u16 {
        ((self.data[pos+1] as u16) << 8) | (self.data[pos] as u16)
    }

    pub fn write_u32(&mut self, data: u32, pos: usize) {
        self.data[pos] = data as u8;
        self.data[pos+1] = (data>>8) as u8;
        self.data[pos+2] = (data>>16) as u8;
        self.data[pos+3] = (data>>24) as u8;
    }

    pub fn read_u32(&self, pos: usize) -> u32 {
        ((self.data[pos+3] as u32) << 24) | 
        ((self.data[pos+2] as u32) << 16) | 
        ((self.data[pos+1] as u32) << 8) | 
        (self.data[pos] as u32)
    }

    pub fn dump(&self, pos: usize) -> String {
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