use std::os::unix::io::FromRawFd;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut f = unsafe { std::fs::File::from_raw_fd(3) };
    
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    
    println!("{}", buf);
    
    Ok(())
}
