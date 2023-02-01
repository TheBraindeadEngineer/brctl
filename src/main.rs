use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long, default_value_t = -1)]
    brightness: i32,
}

use glob::glob;
use std::fs::read_to_string;
use std::process::Command;

fn get_where() -> String {
    for entry in glob("/sys/class/backlight/*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => return path.display().to_string(),
            Err(err) => eprintln!("{err:?}"),
        }
    }
    "".to_string()
}

fn read_max(location: &str) -> i32 {
    let binding = read_to_string(location).unwrap();
    let content = binding.trim();
    content.parse::<i32>().unwrap()
}

fn read_brightness(location: String) -> i32 {
    let location = location + "/max_brightness";
    let binding = read_to_string(&location).unwrap();
    let content = binding.trim();
    let max = read_max(&location);
    (content.parse::<i32>().unwrap() / max) * 100
}

fn get_max() -> i32 {
    for entry in glob(&(get_where() + "/max_brightness")).expect("failed to match glob pattern") {
        match entry {
            Ok(path) => {
                return read_max(&path.display().to_string());
            }
            Err(err) => eprintln!("{err:?}"),
        }
    }
    0
}

fn set_to(percent: i32) {
    let max = get_max();
    let location = get_where();
    let value = (max / 100) * percent;
    Command::new("sh")
        .arg("-c")
        .arg(format!("echo {value} > {location}/brightness"))
        .output()
        .expect("failed to execute command");
}

fn main() {
    let args = Args::parse();
    if args.brightness > 100 || args.brightness < 1 {
        if args.brightness == -1 {
            println!("current brightness is {}%", read_brightness(get_where()));
            return;
        }
        println!("Brightness has to be between 1 and 100");
        return;
    }
    set_to(args.brightness);
    println!("Brightness has been set to {}%", args.brightness);
}
