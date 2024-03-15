mod bin_file;
mod ram;
mod riscv_cpu;

use ram::Ram;
use riscv_cpu::RiscvCpu;

fn main() {
    let filename = "isa/rv32ui-p-add.bin";

    match bin_file::read_file(filename) {
        Ok(bytes) => {
            let ram = Ram::new(bytes);
            let mut pc: u32 = 0;
            let mut cpu = RiscvCpu::new(pc);
            for i in 0..500 {
                println!("current pc: {:x}", pc);
                pc = cpu.tick(ram.read_u32(pc as usize));
            }
        },
        Err(e) => {
            println!("文件读取错误, {}", e);
        }
    }
}
