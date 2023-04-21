
use windows::Win32::System::Console;
use hook::{HasEmote, initialize_skin_hook};

mod hook;
mod offset;

const DLL_PROCESS_DETACH: u32 = 0;
const DLL_PROCESS_ATTACH: u32 = 1;

#[no_mangle]
unsafe extern "stdcall" fn DllMain(
    hinst: usize,
    reason: u32,
    _reserved: *mut ()
) -> bool {
    
    match reason {
        DLL_PROCESS_ATTACH => { 
            std::thread::spawn(move || unsafe { main_thread(hinst) });
            return true
        },
        DLL_PROCESS_DETACH => {
            println!("DLL Ejected");
            return true
        },
        _ => return true
    } 
}



unsafe fn main_thread(_hinst: usize) {
    unsafe { Console::AllocConsole() };

    // Initialize skin hook
    initialize_skin_hook();
    
    // Enable HasEmote hook

    HasEmote
        .enable()
        .unwrap();

    println!("Hello World from Rust");
}
