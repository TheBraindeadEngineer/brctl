use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long, default_value_t = -1)]
    brightness: i32,
}

use std::fs::{read, read_dir};
use std::process::Command;

fn get_where() -> String {
    read_dir("/sys/class/backlight")
        .expect("this doesnt seem to be arch")
        .nth(0)
        .unwrap()
        .unwrap()
        .path()
        .to_str()
        .unwrap()
        .to_string()
}

fn read_max(location: &str) -> i32 {
    use std::str;
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("cat {location}"))
        .output()
        .expect("failed to get max brightness");
    i32::from_str_radix(str::from_utf8(&output.stdout).unwrap().trim(), 10).unwrap()
}

fn read_brightness(location: String) -> i32 {
    use std::str;
    let location = location + "/max_brightness";
    let content = read(&location).unwrap();
    let content = str::from_utf8(&content).unwrap().trim();
    let max = read_max(&location);
    (i32::from_str_radix(content, 10).unwrap() / max) * 100
}

fn get_max() -> i32 {
    let location = get_where();
    for entry in read_dir(location).unwrap() {
        let entry = entry.unwrap();
        if entry.path().to_str().unwrap().contains("max_brightness") {
            return read_max(entry.path().to_str().unwrap());
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
