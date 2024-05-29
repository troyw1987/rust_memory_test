use std::collections::HashMap;

use sysinfo::System;

fn main() {
    let procname = "kitty";

    let sys = System::new_all();
    let mut processes: HashMap<&str, u32> = HashMap::new();

    for (pid, process) in sys.processes() {
        processes.insert(process.name(), pid.as_u32());
    }

    let found_pid = processes.get(&procname).copied().unwrap_or(0);

    println!("pid -> {}", found_pid);
}
