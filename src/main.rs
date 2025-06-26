use colored::*;
use std::env;
use std::fs;
use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use whoami;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    print_system_info(&sys);
}

fn print_system_info(sys: &System) {
    let username = whoami::username();
    let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string());
    
    println!();
    println!("    {}{}{}{}@{}{}{}", 
             "[".bright_blue().bold(),
             username.bright_green().bold(),
             "]".bright_blue().bold(),
             "-".bright_white().bold(),
             "[".bright_blue().bold(),
             hostname.bright_green().bold(),
             "]".bright_blue().bold()
    );
    
    let separator = "─".repeat(username.len() + hostname.len() + 7);
    println!("    {}", separator.bright_blue().bold());
    
    // OS
    let os_name = get_os_info();
    println!("    {}: {}", "OS".bright_blue().bold(), os_name);
    
    // Kernel
    let kernel = sys.kernel_version().unwrap_or_else(|| "Unknown".to_string());
    println!("    {}: {}", "Kernel".bright_blue().bold(), kernel);
    
    // Uptime
    let uptime = format_uptime(sys.uptime());
    println!("    {}: {}", "Uptime".bright_blue().bold(), uptime);
    
    // Shell
    let shell = get_shell();
    println!("    {}: {}", "Shell".bright_blue().bold(), shell);
    
    // CPU
    if let Some(cpu) = sys.cpus().first() {
        let cpu_name = cpu.brand().trim();
        let cpu_usage = format!("{:.1}%", cpu.cpu_usage());
        println!("    {}: {} ({})", "CPU".bright_blue().bold(), cpu_name, cpu_usage);
    }
    
    // Memory
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_usage = format!("{:.1} GiB / {:.1} GiB ({:.1}%)",
                              used_memory as f64 / 1024.0 / 1024.0 / 1024.0,
                              total_memory as f64 / 1024.0 / 1024.0 / 1024.0,
                              (used_memory as f64 / total_memory as f64) * 100.0);
    println!("    {}: {}", "Memory".bright_blue().bold(), memory_usage);
    
    // Disk
    let disk_info = get_disk_info(sys);
    println!("    {}: {}", "Disk (/)".bright_blue().bold(), disk_info);
    
    // Colors
    println!();
    print!("    {}: ", "Colors".bright_blue().bold());
    for color in &["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"] {
        print!("{}", "███".color(*color));
    }
    println!();
    
    println!();
}

fn get_os_info() -> String {
    // Tenta ler /etc/os-release
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line.split('=').nth(1)
                    .unwrap_or("Unknown")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }
    
    // Fallback para whoami
    whoami::distro()
}

fn get_shell() -> String {
    env::var("SHELL")
        .unwrap_or_else(|_| "Unknown".to_string())
        .split('/')
        .last()
        .unwrap_or("Unknown")
        .to_string()
}

fn format_uptime(uptime_seconds: u64) -> String {
    let days = uptime_seconds / 86400;
    let hours = (uptime_seconds % 86400) / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    
    let mut parts = Vec::new();
    
    if days > 0 {
        parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
    }
    if hours > 0 {
        parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
    }
    if minutes > 0 {
        parts.push(format!("{} min{}", minutes, if minutes == 1 { "" } else { "s" }));
    }
    
    if parts.is_empty() {
        "< 1 min".to_string()
    } else {
        parts.join(", ")
    }
}

fn get_disk_info(sys: &System) -> String {
    for disk in sys.disks() {
        if disk.mount_point().to_str() == Some("/") {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;
            
            return format!("{:.1} GiB / {:.1} GiB ({:.1}%)",
                          used as f64 / 1024.0 / 1024.0 / 1024.0,
                          total as f64 / 1024.0 / 1024.0 / 1024.0,
                          (used as f64 / total as f64) * 100.0);
        }
    }
    "Unknown".to_string()
}
