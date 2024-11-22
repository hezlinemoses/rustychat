use std::mem;

use libc::{self, fd_set, select, FD_CLR, FD_SET, FD_SETSIZE, FD_ZERO};


// #[repr(transparent)]
#[derive(Debug)]
pub struct FdSet(libc::fd_set);

pub fn run(){
    
    let mut x: fd_set = unsafe{mem::MaybeUninit::zeroed().assume_init()};
    unsafe {
        println!("{:?}",x);
        FD_ZERO(&mut x);
        println!("{x:?}");
        for i in 0..25{
            FD_SET(i, &mut x);
        }
        FD_SET(10000, &mut x);
        FD_CLR(20, &mut x);
    }
}
