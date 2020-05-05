use std::ffi::OsString;
use std::process::Command;

#[test]
#[cfg(unix)]
fn test_invalid_unicode_on_unix() {
    use std::os::unix::ffi::OsStringExt;
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_test"));
    cmd.arg("abcdef");
    cmd.arg(OsString::from_vec(b"abc\xffdef".to_vec()));
    let status = cmd.status().unwrap();
    assert!(status.success());
}

#[test]
#[cfg(windows)]
fn test_invalid_unicode_on_windows() {
    use std::os::windows::ffi::OsStringExt;
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_test"));
    cmd.arg("abcdef");
    let surrogate_char = 0xDC00;
    let bad_unicode_wchars = [
        'a' as u16,
        'b' as u16,
        'c' as u16,
        surrogate_char,
        'd' as u16,
        'e' as u16,
        'f' as u16,
    ];
    let bad_osstring = OsString::from_wide(&bad_unicode_wchars);
    cmd.arg(bad_osstring);
    let status = cmd.status().unwrap();
    assert!(status.success());
}
