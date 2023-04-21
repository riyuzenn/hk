/// Store all the hooks here. Take a look at `skin.rs`

use windows::{
    s,
    Win32::System::LibraryLoader::GetModuleHandleA,
};

pub use self::skin::*;

pub mod skin;


/// Get the pointer of the method given the address
/// Works with Unity game using IL2CPP.
pub fn get_method_ptr(offset: usize) -> Result<*const (), ()> {
    let handle = unsafe { GetModuleHandleA(s!("GameAssembly.dll")) };

    let addr = (handle.unwrap().0 as usize) +offset;
    Ok(addr as *const ())
    
}
