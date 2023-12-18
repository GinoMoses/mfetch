use std::{fs, 
        process::Command,
        collections::HashMap,
        env,  
};
use colored::Colorize;
use toml;
use serde::Deserialize;

fn main() {
    let mut args: Vec<String> = Vec::new();
    let config: Config;
    // temporary solution, it works for now
    let defualt_config: Config = toml::from_str("
        display = [
            'OS',
            'Host',
            'Kernel',
            'Uptime',
            'Shell',
            'Memory',
            'CPU',
            'GPU',
        ]

        default_ascii = ['what', 'is', 'this', 'ascii', 'art', 'thing', 'lol']
        ").unwrap();
    if let Ok(home_directory) = env::var("HOME") {
        config = toml::from_str(&fs::read_to_string(format!("{}/.config/mfetch/config.toml", home_directory)).unwrap()).unwrap();
    } else {
        config = defualt_config;
    }

    let user = Command::new("sh")
        .arg("-c")
        .arg("whoami")
        .output()
        .expect("failed to execute process (user fetch)");
    args.push(String::from_utf8_lossy(&user.stdout).trim().to_string());

    let hostname = Command::new("sh")
        .arg("-c")
        .arg("uname -n")
        .output()
        .expect("failed to execute process (hostname fetch)");
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
        .expect("failed to execute process (kernel fetch)");
    args.push(String::from_utf8_lossy(&kernel.stdout).trim().to_string());
    
    let uptime = Command::new("sh")
        .arg("-c")
        .arg("uptime -p")
        .output()
        .expect("failed to execute process (uptime fetch)");
    args.push(String::from_utf8_lossy(&uptime.stdout).trim().to_string());

    let shell = Command::new("sh")
        .arg("-c")
        .arg("basename $SHELL")
        .output()
        .expect("failed to execute process (shell fetch)");
    args.push(String::from_utf8_lossy(&shell.stdout).trim().to_string());
   
    let mut memory = String::new();
    let mut memused: u32 = 0;
    match &fs::read_to_string("/proc/meminfo") {
        Ok(meminfo) => {
            for line in meminfo.lines() {
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
                } else {
                    memory.push_str("");
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
            if args.len() < 9 {
                args.push("".to_string());
            }
        }
        Err(_) => args.push("".to_string()),
    }
   
    // for some reason lspci takes a while to execute
    // I don't think there's a file like /proc/cpuinfo for gpus
    // imma look into it later
    let gpu = Command::new("sh")
        .arg("-c")
        .arg("lspci")
        .output()
        .expect("failed to execute process (gpu fetch)");
    for line in String::from_utf8_lossy(&gpu.stdout).trim().to_string().lines() {
        if line.contains("VGA") {
            let line = &line.split(":").collect::<Vec<&str>>();
            let line = &line[2].trim();
            args.push(line.to_string());
            break;
        }

        if line.contains("3D controller") {
            let line = &line.split(":").collect::<Vec<&str>>();
            let line = &line[3].trim();
            args.push(line.to_string());
            break;
        }
    }
    
    if args.len() < 10 {
        args.push("".to_string());
    }

    Arguments::display(&Arguments::build(&args), Arguments::hashmap_build(&Arguments::build(&args)), config);
}

#[derive(Deserialize)]
struct Config {
    display: Vec<String>,
    default_ascii: Vec<String>,
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
    
    fn hashmap_build(&self) -> HashMap<String, String> {
        let mut hashmap = HashMap::new();
        hashmap.insert("os".to_string(), self.os.clone());
        hashmap.insert("host".to_string(), self.host.clone());
        hashmap.insert("kernel".to_string(), self.kernel.clone());
        hashmap.insert("uptime".to_string(), self.uptime.clone());
        hashmap.insert("shell".to_string(), self.shell.clone());
        hashmap.insert("memory".to_string(), self.memory.clone());
        hashmap.insert("cpu".to_string(), self.cpu.clone());
        hashmap.insert("gpu".to_string(), self.gpu.clone());
        hashmap
    }

    fn display(&self, options: HashMap<String, String>, config: Config) {
        let mut ascii: Vec<String> = ascii_insert(&config).lines().map(|line| line.blue().bold().to_string()).collect();
        let min_lines = config.display.len();
        let max_line_length = ascii.iter().map(|line| line.len()).max().unwrap_or(0);
        
        for string in &mut ascii {
            let fill = max_line_length - string.len();
            string.push_str(&" ".repeat(fill));
        }

        let satisfy_min = min_lines.saturating_sub(ascii.len());
        let filler = " ".repeat(max_line_length);

        for _ in 0..satisfy_min {
            ascii.push(filler.clone());
        
        }

        ascii[0] += format!("   {}@{}", (self.user).blue().bold(), (self.hostname).blue().bold()).as_str();
        ascii[1] += "   ";
        ascii[1] += "-".repeat(self.user.len() + self.hostname.len() + 1).as_str();
        let mut i = 0;
        
        for e in config.display {
            if options.get(&e.to_lowercase()).unwrap() != "" {
                ascii[i + 2] += format!("   {} >> {}", &e.blue().bold(), options.get(&e.to_lowercase()).unwrap()).as_str();
                i += 1;
            }
        }

        for line in ascii {
            println!("{}", line);
        }
    }
}
fn ascii_insert(config: &Config) -> String { 
    config.default_ascii.join("\n")
}
