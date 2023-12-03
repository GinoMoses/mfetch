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

    let os = &fs::read_to_string("/etc/os-release")
        .expect("Something went wrong reading the file");
    for line in os.lines() {
        if line.contains("PRETTY_NAME") {
            let os = line.split("=").collect::<Vec<&str>>();
            args.push(os[1].to_string().replace("\"", ""));
            break;
        }
    }

    let host = &fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
        .expect("Something went wrong reading the file");
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

    Arguments::display(&Arguments::build(&args));
}

struct Arguments {
    user: String,
    hostname: String,
    os: String,
    host: String,
    kernel: String,
    uptime: String,
}

impl Arguments {
    fn build(args: &[String]) -> Self {
        let user = args[0].clone();
        let hostname = args[1].clone();
        let os = args[2].clone();
        let host = args[3].clone();
        let kernel = args[4].clone();
        let uptime = args[5].clone();

        Arguments { hostname, user, os, host, kernel, uptime }
    }
    fn display(&self) {
        println!("{}@{}", (self.user).blue().bold(), (self.hostname).blue().bold());
        println!("-------------------------");
        println!("{} >> {}", "OS".blue().bold(), self.os);
        println!("{} >> {}", "Host".blue().bold(), self.host);
        println!("{} >> {}", "Kernel".blue().bold(), self.kernel);
        println!("{} >> {}", "Uptime".blue().bold(), self.uptime);
        
        // println!("{}", "  ".on_blue());
    }
}
