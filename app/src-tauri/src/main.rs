// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use interactive_keyboard_lib::prelude::*;

fn main() -> Result<()> {
    interactive_keyboard_lib::run(utils::get_port())
}
