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


    println!("memory: {}MiB / {}MiB", used / 1024, total / 1024);

   
    let shell = env::var("SHELL").unwrap();
   
println!("shell: {}", shell);

 let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap();
  
 
 
    for line in cpuinfo.lines() {
 
        if line.starts_with("model name") {
            let parts: Vec<&str> = line.split(':').collect();

            println!("cpu: {}", parts[1].trim());
             break;
     
   }
    }

}

