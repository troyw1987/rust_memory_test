use std::collections::HashMap;

use sysinfo::System;

fn getpidfromstring(procname: &str) -> u32 {
    let sys = System::new_all();
    let mut processes: HashMap<&str, u32> = HashMap::new();

    for (pid, process) in sys.processes() {
        processes.insert(process.name(), pid.as_u32());
    }

    let found_pid = processes.get(&procname).copied().unwrap_or(0);

    found_pid
}

fn main() {
    let procname = "Shigatari";
    let found_pid = getpidfromstring(procname);

    println!("pid -> {}", found_pid);
}
