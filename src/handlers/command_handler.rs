use std::process::Command;

pub fn command_handler(cmd_args: Vec<&str>) {
    Command::new(cmd_args[0]).args(&cmd_args[1..]).spawn().unwrap();
}