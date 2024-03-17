mod bin_file;
mod ram;
mod riscv_cpu;

use ram::Ram;
use riscv_cpu::RiscvCpu;

fn main() {
    let filenames = [
        "isa/rv32ui-p-add.bin", 
        "isa/rv32ui-p-addi.bin",
        "isa/rv32ui-p-and.bin",
        "isa/rv32ui-p-andi.bin",
        "isa/rv32ui-p-auipc.bin",
        "isa/rv32ui-p-beq.bin",
        "isa/rv32ui-p-bge.bin",
        "isa/rv32ui-p-bgeu.bin",
        "isa/rv32ui-p-blt.bin",
        "isa/rv32ui-p-bltu.bin",
        "isa/rv32ui-p-bne.bin",
        "isa/rv32ui-p-fence_i.bin",
        "isa/rv32ui-p-jal.bin",
        "isa/rv32ui-p-jalr.bin",
        "isa/rv32ui-p-lb.bin",
        "isa/rv32ui-p-lbu.bin",
        "isa/rv32ui-p-lh.bin",
        "isa/rv32ui-p-lhu.bin",
        "isa/rv32ui-p-lui.bin",
        "isa/rv32ui-p-lw.bin",
        "isa/rv32ui-p-or.bin",
        "isa/rv32ui-p-ori.bin",
        "isa/rv32ui-p-sb.bin",
        "isa/rv32ui-p-sh.bin",
        "isa/rv32ui-p-simple.bin",
        "isa/rv32ui-p-sll.bin",
        "isa/rv32ui-p-slli.bin",
        "isa/rv32ui-p-slt.bin",
        "isa/rv32ui-p-slti.bin",
        "isa/rv32ui-p-sltiu.bin",
        "isa/rv32ui-p-sltu.bin",
        "isa/rv32ui-p-sra.bin",
        "isa/rv32ui-p-srai.bin",
        "isa/rv32ui-p-srl.bin",
        "isa/rv32ui-p-srli.bin",
        "isa/rv32ui-p-sub.bin",
        "isa/rv32ui-p-sw.bin",
        "isa/rv32ui-p-xor.bin",
        "isa/rv32ui-p-xori.bin",
        "isa/rv32um-p-div.bin",
        "isa/rv32um-p-divu.bin",
        "isa/rv32um-p-mul.bin",
        "isa/rv32um-p-mulh.bin",
        "isa/rv32um-p-mulhsu.bin",
        "isa/rv32um-p-mulhu.bin",
        "isa/rv32um-p-rem.bin",
        "isa/rv32um-p-remu.bin",
    ];

    let mut failed = 0;
    let mut failed_filename = Vec::new();
    let mut not_complete = 0;
    let mut not_complete_filename = Vec::new();
    for filename in filenames {
        println!("start read {filename}");
        match bin_file::read_file(filename) {
            Ok(bytes) => {
                let ram = Ram::new(bytes);
                let mut cpu = RiscvCpu::new(0, ram);
                let mut exit_loop = 0;
                for _ in 0..500 {
                    cpu.tick();

                    let s10 = cpu.get_rs(26);
                    if s10 == 1 {
                        exit_loop += 1;
                        if exit_loop > 10 { 
                            println!("loop break at {}", cpu.get_tick_cnt());
                            break;
                        }
                    }
                }
    
                let s10 = cpu.get_rs(26);
                let s11 = cpu.get_rs(27);
                if s10 == 1 { 
                    if s11 == 1 {
                        println!("{filename} test Ok!!!");
                    } else {
                        println!("{filename} test Failed!!!");
                        failed += 1;
                        failed_filename.push(filename.to_string());
                    }
                } else {
                    println!("{filename} test not completed!!!");
                    not_complete += 1;
                    not_complete_filename.push(filename.to_string());
                }
            },
            Err(e) => {
                println!("文件读取错误, {}", e);
                break;
            }
        }
    }
    println!("failed {failed}: ");
    for name in failed_filename {
        println!("{name}.");
    }
    println!("not_complete {not_complete}: ");
    for name in not_complete_filename {
        println!("{name}: ");
    }
    println!("successful {}.", filenames.len() - failed - not_complete);
}
