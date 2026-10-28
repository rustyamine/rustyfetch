use std::env;

use std::fs;


fn get_os() -> String {

    let osfile = fs::read_to_string("/etc/os-release").unwrap();

    let mut name = String::new();

    for line in osfile.lines() {
    if line.starts_with("PRETTY_NAME=") {
     name = line.replace("PRETTY_NAME=", "").replace("\"", "");
    
    }
    
}
name
}

fn get_kernel() -> String {

    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap();
    kernel.trim().to_string()

}

fn get_uptime() -> String {

    let up = fs::read_to_string("/proc/uptime").unwrap();

    let secs: f64 = up.split_whitespace().next().unwrap().parse().unwrap();
    let mins = (secs / 60.0) as u64;

    format!("{}h {}m", mins / 60, mins % 60)
}

 
 fn get_mem() -> String {
 
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();
 
    let mut total: u64 = 0;
    let mut free: u64 = 0;

    for line in meminfo.lines() {
  
        if line.starts_with("MemTotal") {
  
             let parts: Vec<&str> = line.split_whitespace().collect();
            total = parts[1].parse().unwrap();
        }
   
         if line.starts_with("MemAvailable") {
    
             let parts: Vec<&str> = line.split_whitespace().collect();
              free = parts[1].parse().unwrap();
       
            }
    }

    let used = total - free;
   
    format!("{}MiB / {}MiB", used / 1024, total / 1024)
}

fn get_cpu() -> String {
   
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap();
    let mut cpu = String::new();
    for line in cpuinfo.lines() {
   
        if line.starts_with("model name") {
            let parts: Vec<&str> = line.split(':').collect();
   
            cpu = parts[1].trim().to_string();
   
            break;
        }
    }
   
    cpu
}

fn main() {
   
     let user = env::var("USER").unwrap();
     let host = fs::read_to_string("/proc/sys/kernel/hostname").unwrap();
     let shell = env::var("SHELL").unwrap();

    
     println!("{}@{}", user, host.trim());
    println!("os: {}", get_os());
    println!("kernel: {}", get_kernel());
    println!("uptime: {}", get_uptime());
    println!("memory: {}", get_mem());
    println!("shell: {}", shell);
    println!("cpu: {}", get_cpu());
}