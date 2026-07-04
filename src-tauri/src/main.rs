#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Force cargo rebuild for icon
    rifly_lib::run()
}
