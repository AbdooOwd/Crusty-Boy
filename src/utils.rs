use std::{fs::{self, OpenOptions}, io::Write};

use time::Duration;

pub const PANIC_HANDLE: bool = true;
#[allow(unused)]
pub const NO_FLAGS_MESSAGE: &str = "Instruction doesn't support 'F' register (FLAGS)";
pub const DEBUG_ENABLED: bool = true;

const LOG_PATH: &str = "data/logs.txt";

#[allow(unused)]
pub fn handle_error(message: &str) -> () {
    if PANIC_HANDLE {
        println!("{}", message);
    } else {
        panic!("{}", message);
    }
}

pub fn panic_log(message: &str) -> ! {
    debug_logs(&message);
    panic!("{}", message);
}

pub fn reset_logs() -> () {
    let log_exist = fs::exists(LOG_PATH).expect("Couldn't check existence of LOGS");
    if !log_exist {
        println!("Logs doesn't exist!");
        return;
    }
    fs::remove_file(LOG_PATH).expect("Couldn't Remove Log File");
}

pub fn debug_logs(log: &str) -> () {
    let mut file = OpenOptions::new().append(true).create(true).open(LOG_PATH).expect("Couldn't do logs");

    let _ = file.write_fmt(format_args!("{}\n", log));
}

pub fn log(log: &str) -> () {
    debug_logs(log);
    println!("{log}");
}



pub fn delay(ms: u32) -> () {
    std::thread::sleep(std::time::Duration::from_millis(ms.into()));
}