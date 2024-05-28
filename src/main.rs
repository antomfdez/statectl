use std::env;
use std::convert::AsRef;
use std::process::Command;
use std::process::exit;
use colored::Colorize;

fn hibernate() {
    println!("\n{}\n", "Hibernating system...".cyan().bold());
    let _command = Command::new("bash")
        .arg("-c")
        .arg("sleep 3 && echo disk | tee /sys/power/state")
        .spawn();
}

fn sleep() {
    println!("\n{}\n", "Sleeping system...".cyan().bold());
    let _command = Command::new("bash")
        .arg("-c")
        .arg("sleep 3 && echo deep | tee /sys/power/mem_sleep && echo mem | sudo tee /sys/power/state")
        .spawn();
}

fn freeze() {
    println!("\n{}\n", "Freezing system...".cyan().bold());
    let _command = Command::new("bash")
        .arg("-c")
        .arg("sleep 3 && echo freeze | tee /sys/power/state")
        .spawn();
}

fn out_of_range(name: String) {
    println!("\n{}\n","Not a valid mode...!".yellow().bold());
    println!("{} {} {}\n", "Usage:".yellow().bold(), name.bold(), "[hibernate,sleep,freeze]".green().bold().italic());
    exit(2);
}

fn state(mode: String, name: String) {
    match mode.as_ref() {
        "hibernate" => hibernate(),
        "sleep" => sleep(),
        "freeze" => freeze(),
        "-h" | "--help" => println!("\n{} {} {}\n", "Usage:".yellow().bold(), name.bold(), "[hibernate,sleep,freeze]".green().bold().italic()),
        _ => out_of_range(name),
    }
}

fn main() {
    match env::var("USER") {
        Err(e) => {
            println!("Something went wrong: {:?}", e);
            exit(2);
        }
        Ok(name) => {
            if name != "root" {
                println!("\n{}\n", "Must be root...!".red().bold());
                exit(1);
            }
        }
    }
    let args: Vec<String> = env::args().collect();
    let name: String = args[0].to_string();
    if args.len() == 1 {
        println!("\n{}\n", "Mode needed...!".yellow().bold());
        println!("{} {} {}\n", "Usage:".yellow().bold(), name.bold(), "[hibernate,sleep,freeze]".green().bold().italic());
        exit(2);
    } else {
        let mode: String = args[1].to_string().to_lowercase();
        state(mode, name);
    }

}
