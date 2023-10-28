use std::env;
use std::process::Command;
use std::ptr::null_mut;
use winapi::um::winuser::MessageBoxA;
use winapi::um::winuser::MB_OK;

fn main() {
    // Fetch the command from the environment variable
    let command = env::var("kprotonCommand").unwrap_or_default();

    if command.is_empty() {
        unsafe {
            MessageBoxA(null_mut(), "kprotonCommand is not set or is empty.".as_ptr() as *const i8, "Error".as_ptr() as *const i8, MB_OK);
        }
        return;
    }

    // Execute the command using cmd.exe
    Command::new("cmd")
        .args(&["/C", &command])
        .status()
        .expect("Failed to execute command.");
}
