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
