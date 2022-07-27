use std::ffi::{CStr, c_void};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::raw::c_char;


fn log_sslkeylogfile_line_help(line_ptr: *const c_char) -> Option<()>
{
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/openssl_keylogfile.txt")
        .ok()?;

    let line_str = unsafe { CStr::from_ptr(line_ptr) };
    let line = line_str.to_str().ok()?;

    writeln!(f, "{}", line).ok()?;
    return Some(());
}


#[no_mangle]
pub extern "C" fn log_sslkeylogfile_line(_ssl: *const c_void, line: *const c_char)
{
    log_sslkeylogfile_line_help(line);
}
