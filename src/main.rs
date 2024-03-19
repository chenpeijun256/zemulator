mod bin_file;
mod mem;
mod com_reg;
mod csr_reg;
mod riscv_cpu;
mod perips;
mod config;

use riscv_cpu::RiscvCpu;

fn test_isa() {
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
                let mut cpu: RiscvCpu = config::build_soc("".to_owned());
                cpu.fill_mem(0, bytes, 0);

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

fn split_string(line: String) -> Vec<String> {
    let mut res = Vec::new();

    line.trim().split_ascii_whitespace().for_each(|elem| {
        res.push(elem.to_owned());
    });
    return res;
}

fn parse_i32(n_str: &str) -> i32 {
    match i32::from_str_radix(n_str, 10) {
        Ok(steps) => steps,
        Err(_) => i32::MIN,
    }
}

fn parse_hex_u32(n_str: &str) -> u32 {
    match u32::from_str_radix(n_str, 16) {
        Ok(steps) => steps,
        Err(_) => 0,
    }
}

fn test_one_file(filename: &String, mut steps: i32) {
    println!("start read {filename}");
    match bin_file::read_file(filename) {
        Ok(bytes) => {
            let mut cpu: RiscvCpu = config::build_soc("rv32im.cfg".to_owned());
            cpu.fill_mem(0, bytes, 0);

            loop {
                if steps >= 0 {
                    while steps > 0 {
                        cpu.tick();
                        steps -= 1;
                    }

                    let mut key = String::new();
                    match std::io::stdin().read_line(&mut key) {
                        Ok(_) => {
                            // println!("{n} bytes read.");
                            // println!("key = {}.", key.trim());
                            let cmds = split_string(key);
                            if cmds.len() > 0 {
                                if cmds[0] == "q" {
                                    break;
                                } else if cmds[0] == "s" {
                                    if cmds.len() > 1 {
                                        steps = parse_i32(&cmds[1]);
                                    } else {
                                        steps = 1;
                                    }
                                } else if cmds[0] == "r" {
                                    steps = 1;
                                } else if cmds[0] == "i" {
                                    println!("insert breakpoint.");
                                    steps = 0;
                                } else if cmds[0] == "p" {
                                    if cmds.len() > 1 {
                                        if cmds[1] == "mem" {
                                            if cmds.len() > 2{
                                                cpu.print_mem(parse_hex_u32(&cmds[2]));
                                            } else {
                                                cpu.print_mem(0);
                                            }
                                        } else if cmds[1] == "csr" {
                                            cpu.print_csr();
                                        } else {
                                            cpu.print_reg();
                                        }
                                    } else {
                                        println!("e.g. p reg/csr.");
                                        println!("     p mem xxx(hex).");
                                    }
                                    steps = 0;
                                } else {
                                    println!("command can not found.");
                                    steps = 0;
                                }
                            } else {
                                println!("command can not found.");
                                steps = 0;
                            }
                        },
                        Err(e) => {
                            println!("input error {e}.")
                        },
                    }
                } else {
                    cpu.tick();
                }
            }


            println!("{filename} test completed!!! tick cnt: {}.", cpu.get_tick_cnt());
        },
        Err(e) => {
            println!("文件读取错误, {}", e);
        }
    }
}

fn main() {
    let args:Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        if args[1] == "isa" {
            test_isa();
        } else {
            if args.len() > 2 {
                if args[2] =="-d" {
                    test_one_file(&args[1], 0);
                } else {
                    match args[2].parse::<i32>() {
                        Ok(steps) => test_one_file(&args[1], steps),
                        Err(e) => println!("arg format error. {e}"),
                    };
                }
            } else {
                test_one_file(&args[1], -1);
            }
        }
    } else {
        println!("Please input with following format:");
        println!("1. test all isa file: zemulator isa.");
        println!("2. run and stop at start: zemulator filename -d.");
        println!("3. run and stop at xxx steps: zemulator filename xxx.");
        println!("4. run with no stop: zemulator filename.");
        println!("--------------------------------");
    }
}
