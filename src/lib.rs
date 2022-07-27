use std::io::Write;
use std::ffi::{CStr, c_void};
use std::fs::OpenOptions;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn print_sslkeylogfile_line(_ssl: *const c_void, line: *const c_char)
{
    let mut log_file = OpenOptions::new()
        .append(true)
        .open("/tmp/openssl_keylogfile.txt");
    if log_file.is_err() {
        return;
    }
    let Ok(mut f) = log_file;

    let slice = unsafe { CStr::from_ptr(line) };
    let slice_r = slice.to_str();
    if slice_r.is_err() {
        return;
    }
    let Ok(printable_line) = slice_r;
    writeln!(f, "{}", printable_line);
}
