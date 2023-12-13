use std::process::Command;
use std::fs;
use colored::Colorize;

fn main() {
    let mut args: Vec<String> = Vec::new();

    let user = Command::new("sh")
        .arg("-c")
        .arg("whoami")
        .output()
        .expect("failed to execute process");
    args.push(String::from_utf8_lossy(&user.stdout).trim().to_string());

    let hostname = Command::new("sh")
        .arg("-c")
        .arg("uname -n")
        .output()
        .expect("failed to execute process");
    args.push(String::from_utf8_lossy(&hostname.stdout).trim().to_string());

    match fs::read_to_string("/etc/os-release") {
        Ok(os) => {
            for line in os.lines() {
                if line.contains("PRETTY_NAME") {
                    let os = line.split("=").collect::<Vec<&str>>();
                    args.push(os[1].to_string().replace("\"", ""));
                    break;
                }
            }
        }
        Err(_) => args.push("".to_string()),
    };

    let host = &fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
        .unwrap_or_else(|_| "".to_string());
    args.push(host.trim().to_string());

    let kernel = Command::new("sh")
        .arg("-c")
        .arg("uname -r")
        .output()
        .expect("failed to execute process");
    args.push(String::from_utf8_lossy(&kernel.stdout).trim().to_string());
    
    let uptime = Command::new("sh")
        .arg("-c")
        .arg("uptime -p")
        .output()
        .expect("failed to execute process");
    args.push(String::from_utf8_lossy(&uptime.stdout).trim().to_string());

    let shell = Command::new("sh")
        .arg("-c")
        .arg("basename $SHELL")
        .output()
        .expect("failed to execute process");
    args.push(String::from_utf8_lossy(&shell.stdout).trim().to_string());
    
    let meminfo = &fs::read_to_string("/proc/meminfo");
    let mut memory = String::new();
    let mut memused: u32 = 0;
    match meminfo {
        Ok(mem) => {
            for line in mem.lines() {
                if line.contains("MemTotal") {
                    let memtotal = &line.split(":").collect::<Vec<&str>>();
                    let memtotal = &memtotal[1].split("kB").collect::<Vec<&str>>();
                    let memtotal = &memtotal[0].trim();
                    let memtotal = &memtotal.parse::<u32>().unwrap() / 1024;
                    memused = memtotal;
                    memory.push_str(&memtotal.to_string());
                }
                if line.contains("MemAvailable") {
                    let memav = &line.split(":").collect::<Vec<&str>>();
                    let memav = &memav[1].split("kB").collect::<Vec<&str>>();
                    let memav = &memav[0].trim();
                    let memav = &memav.parse::<u32>().unwrap() / 1024;
                    memused -= &memav;
                    memory = format!("{}M / {}M", memused, memory);
                }
            }
        }
        Err(_) => memory.push_str(""),
    };
    args.push(memory.trim().to_string());
    
    match &fs::read_to_string("/proc/cpuinfo") {
        Ok(cpu) => {
            for line in cpu.lines() {
                if line.contains("model name") {
                    let cpu = &line.split(":").collect::<Vec<&str>>();
                    let cpu = &cpu[1].trim();
                    args.push(cpu.to_string());
                    break;
                }
            }
        }
        Err(_) => args.push("".to_string()),
    }
   
    // for some reason lspci takes a while to execute
    // I don't think there's a file like /proc/cpuinfo for gpus
    // imma look into it later
    let gpu = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep VGA | cut -d ':' -f3 | cut -d '(' -f1")
        .output()
        .expect("failed to execute process");
    args.push(String::from_utf8_lossy(&gpu.stdout).trim().to_string());

    Arguments::display(&Arguments::build(&args));
}
struct Arguments {
    user: String,
    hostname: String,
    os: String,
    host: String,
    kernel: String,
    uptime: String,
    shell: String,
    memory: String,
    cpu: String,
    gpu: String,
}

impl Arguments {
    fn build(args: &[String]) -> Self {
        let user = args[0].clone();
        let hostname = args[1].clone();
        let os = args[2].clone();
        let host = args[3].clone();
        let kernel = args[4].clone();
        let uptime = args[5].clone();
        let shell = args[6].clone();
        let memory = args[7].clone();
        let cpu = args[8].clone();
        let gpu = args[9].clone();

        Arguments { hostname, user, os, host, kernel, uptime, shell, memory, cpu, gpu }
    }
    fn display(&self) {
        // there is probably a better way to do this
        // but this is the best I could come up with tbh
        // might fix later idk
        // might honestly refactor the whole idea of using a struct
        println!("{}@{}", (self.user).blue().bold(), (self.hostname).blue().bold());
        println!("-------------------------");
        if self.os != "" {
            println!("{} >> {}", "OS".blue().bold(), self.os);
        }
        if self.host != "" {
            println!("{} >> {}", "Host".blue().bold(), self.host);
        }
        println!("{} >> {}", "Kernel".blue().bold(), self.kernel);
        println!("{} >> {}", "Uptime".blue().bold(), self.uptime);
        println!("{} >> {}", "Shell".blue().bold(), self.shell); 
        println!("{} >> {}", "Memory".blue().bold(), self.memory);
        println!("{} >> {}", "CPU".blue().bold(), self.cpu);
        println!("{} >> {}", "GPU".blue().bold(), self.gpu);
    }
}
