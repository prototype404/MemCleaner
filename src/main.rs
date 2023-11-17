use winapi::um::processthreadsapi::OpenProcess;
use sysinfo::{ProcessExt, System, SystemExt, PidExt};
use winapi::um::winnt::PROCESS_ALL_ACCESS;

#[link(name = "psapi")]
extern {
    fn EmptyWorkingSet(handle: u32) -> u8;
}

fn main() {
    let sys = System::new_all();

    for (pid, process) in sys.processes() {

        unsafe {
            let process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid.as_u32());
            let result: u8 = EmptyWorkingSet(process_handle as u32);

            match result {
                1 => println!("[{pid}] {:?}", process.name()),
                _ => continue
            }
        }
    }
}
