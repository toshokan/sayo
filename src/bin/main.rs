use std::env;
use std::process::Command;
use std::fs::File;
use std::os::unix::process::CommandExt;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut handle = unsafe {
	Command::new(&args[2])
	    .pre_exec(move || {
		use std::os::unix::io::AsRawFd;
		
		let file = File::open(&args[1]).unwrap();
		let fd = file.as_raw_fd();
		
		let flags = libc::fcntl(fd, libc::F_GETFD);
		libc::fcntl(fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC);
		
		std::mem::forget(file);
		
		Ok(())
	    })
	    .spawn()
	    .unwrap()
    };

    handle.wait()?;

    Ok(())
}
