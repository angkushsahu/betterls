pub(crate) fn error_log(message: &str) {
    let error_color = "\x1b[31m";
    eprintln!("{}{}\x1b[0m", error_color, message);
}

#[cfg(windows)]
pub(crate) fn enable_ansi_support() {
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        STD_OUTPUT_HANDLE,
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle.is_null()
            || handle == std::ptr::null_mut()
            || handle == -1_isize as *mut std::ffi::c_void
        {
            return;
        }

        let mut mode = 0;
        if GetConsoleMode(handle, &mut mode) != 0 {
            // Bitwise OR to enable ANSI escape codes (Virtual Terminal Processing)
            mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            SetConsoleMode(handle, mode);
        }
    }
}

#[cfg(not(windows))]
pub(crate) fn enable_ansi_support() {}
