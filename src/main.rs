use std::collections::HashMap;

use process_memory::*;
use sysinfo::System;

fn get_pid(procname: &str) -> process_memory::Pid {
    let sys = System::new_all();
    let mut processes: HashMap<&str, u32> = HashMap::new();

    for (pid, process) in sys.processes() {
        processes.insert(process.name(), pid.as_u32());
    }

    let found_pid = processes.get(&procname).copied().unwrap_or(0);

    found_pid as Pid
}

fn main() {
    let procname = "Shigatari";
    let process_handle = get_pid(procname).try_into_process_handle().unwrap();

    println!("pid -> {}", process_handle.0);

    let mut character_exp = DataMember::<u32>::new(process_handle);
}
