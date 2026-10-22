use std::env;

use std::fs;


fn main() {


    let user = env::var("USER").unwrap();

    let host = fs::read_to_string("/proc/sys/kernel/hostname").unwrap();

    println!("{}@{}", user, host.trim());

    let osfile = fs::read_to_string("/etc/os-release").unwrap();

    for line in osfile.lines() {

   if line.starts_with("PRETTY_NAME=") {

            let name = line.replace("PRETTY_NAME=", "").replace("\"", "");
       println!("os: {}", name);

        }

    }
    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap();
    println!("kernel: {}", kernel.trim());

    let up = fs::read_to_string("/proc/uptime").unwrap();
    let secs: f64 = up.split_whitespace().next().unwrap().parse().unwrap();
    let mins = (secs / 60.0) as u64;
    
println!("uptime: {}h {}m", mins / 60, mins % 60);
}

