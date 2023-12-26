#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use winapi::um::processthreadsapi::OpenProcess;
use sysinfo::System;
use winapi::um::winnt::PROCESS_ALL_ACCESS;

#[tauri::command]
async fn calculate_usage_memory() -> String {
    let mut sys = System::new_all();

    sys.refresh_memory();

    let total_memory: u64 = sys.total_memory();
    let usage_memory: u64 = sys.used_memory();
    let usage_percent = (usage_memory * 100) / total_memory;

    usage_percent.to_string()
}

#[link(name = "psapi")]
extern {
    fn EmptyWorkingSet(handle: u32) -> u8;
}

#[tauri::command]
async fn clean() {
    let sys = System::new_all();

    for (pid, _) in sys.processes() {

        unsafe {
            let process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid.as_u32());
            EmptyWorkingSet(process_handle as u32);
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate_usage_memory,
                                                 clean])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
