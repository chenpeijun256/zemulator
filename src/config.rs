use std::{fs::{self, File}, io::BufReader};
use crate::{mem::Mem, rv32_actor::Rv32Actor};
use crate::perips::Perips;
use crate::rv32_actor::cpu::Rv32Cpu;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CCpu {
    name: String,
    class: String,
    isa: String,
    freq: f32,
}

#[derive(Serialize, Deserialize)]
struct CMem {
    name: String,
    start: u32,
    size: u32,
}

#[derive(Serialize, Deserialize)]
struct CPerips {
    name: String,
    class: u32,
    start: u32,
    size: u32,
    intr: u32,
}

#[derive(Serialize, Deserialize)]
struct CSoc {
    name: String,
    rst_pc: u32,
    cpus: Vec<CCpu>,
    mems: Vec<CMem>,
    perips: Vec<CPerips>,
}

pub fn build_soc(cfg_file: String) -> Rv32Actor {
    let soc_cfg = read_cfg(cfg_file);
    println!("create {} soc with rst pc: {}", soc_cfg.name, soc_cfg.rst_pc);
    let mut soc: Rv32Actor = Rv32Actor::new();

    for cfg in soc_cfg.cpus {
        let cpu = Rv32Cpu::new(cfg.name, soc_cfg.rst_pc, cfg.freq);
        println!("add cpu {:?} to cpu.", cpu);
        soc.add_cpu(cpu);
    }

    for cfg in soc_cfg.mems {
        let mem = Mem::new(cfg.name, cfg.start, cfg.size);
        println!("add mem {:?} to cpu.", mem);
        soc.add_mem(mem);
    }

    for cfg in soc_cfg.perips {
        let p = Perips::new(cfg.name, cfg.start, cfg.size, cfg.intr);
        println!("add perips {:?} to cpu.", p);
        soc.add_perips(p);
    }

    soc
}

fn read_cfg(cfg_file: String) -> CSoc {

    match fs::File::open(cfg_file) {
        Ok(f) => {
            let reader = BufReader::new(f);
            match serde_json::from_reader::<BufReader<File>, CSoc>(reader) {
                Ok(s) => {
                    return s;
                },
                Err(e) => println!("json string read failed. {e}"),
            }
        },
        Err(e) => println!("config file open failed. {e}"),
    };

    return CSoc{name: "default".to_owned(), 
                rst_pc: 0,
                cpus: vec![CCpu{name: "cpu0".to_owned(), class: "rv32".to_owned(), isa: "im".to_owned(), freq: 50.0}], 
                mems: vec![CMem{name: "ram".to_owned(), start: 0, size: 8192}], 
                perips: Vec::new()
            };

    // let json_str = "{\"name\": \"cpu0\", \"freq\": 50.0}";
    // let json = serde_json::from_str(json_str);
    // let mut s = Soc{name: "my_board".to_owned(), cpus: Vec::new()};
    // s.cpus.push(Cpu{name: "cpu0".to_owned(), class: "rv32".to_owned(), isa: "im".to_owned(), freq: 100.0});
    // s.cpus.push(Cpu{name: "cpu1".to_owned(), class: "rv64".to_owned(), isa: "g".to_owned(), freq: 150.0});
    // s.cpus.push(Cpu{name: "cpu2".to_owned(), class: "rv64".to_owned(), isa: "imacfd".to_owned(), freq: 1000.0});

    // let json_ob = serde_json::to_string_pretty::<Soc>(&s);
    // match json_ob {
    //     Ok(json_str) => println!("{}", json_str),
    //     Err(_) => println!("json seri failed."),
    // }
    
}

