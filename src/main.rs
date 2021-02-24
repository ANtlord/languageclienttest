use std::{env::args, fs, ptr, slice};
use std::os::unix::io::AsRawFd;

// First argument is a binary that should be parsed by the `object` library.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_name = args().skip(1).next().ok_or("Executable is not designated")?;
    let fd = fs::File::open(&program_name).map_err(|e| format!("fail openning executable file: {}", e))?;
    let size = fd.metadata().map_err(|e| format!("fail getting metadata: {}", e))?.len() as usize;
    let data: &[u8] = unsafe {
        let res = libc::mmap(ptr::null_mut(), size, libc::PROT_READ, libc::MAP_PRIVATE, fd.as_raw_fd(), 0);
        if libc::MAP_FAILED == res {
            panic!("failed mmap");
        }

        slice::from_raw_parts(res as *const _, size)
    };

     object::File::parse(&data).map_err(|e| format!("fail building an object file: {}", e))?;
     Ok(())
}
