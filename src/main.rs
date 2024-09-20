#![cfg_attr(not(test), windows_subsystem = "windows")]

use std::env;
use std::os::windows::process::CommandExt;
use std::process::Command;

use windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <program> [args...]", args[0]);
        std::process::exit(1);
    }

    let program = &args[1];
    let program_args = &args[2..];

    let status = Command::new(program)
        .args(program_args)
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .expect("Failed to execute the program");

    std::process::exit(status.code().unwrap_or(1));
}
