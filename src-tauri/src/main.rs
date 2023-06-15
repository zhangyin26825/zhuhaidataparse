// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(unused_imports)]  
#![allow(dead_code)]
#![warn(unused_must_use)] 
#[warn(unknown_lints)]
pub mod parse;
pub mod school;
use parse::{parse_dir,clear_data};
use tauri::Manager;

fn main() {
  tauri::Builder::default()
  .setup(|app| {
    #[cfg(debug_assertions)] // only include this code on debug builds
    {
      let window = app.get_window("main").unwrap();
      window.open_devtools();
      // window.close_devtools();
    }
    Ok(())
  })
   .invoke_handler(tauri::generate_handler![parse_dir,clear_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


