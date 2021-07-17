#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{
    chrdev, 
    cstr, 
    file_operations::{File, FileOperations},
    user_ptr::{UserSlicePtrReader, UserSlicePtrWriter},
    error::Error,
};

module! {
    type: RustChrdev,
    name: b"rust_chrdev_learning",
    author: b"kyuta3579",
    description: b"Rust character device learning code",
    license: b"",
    params: {
    },
}

#[derive(Default)]
struct RustFile;

impl FileOperations for RustFile {
    kernel::declare_file_operations!(read, write);

    fn read(&self, _file: &File, _data: &mut UserSlicePtrWriter, _offset: u64) -> KernelResult<usize> {
        pr_info!("Rust character device: read\n");
        let s = "OOPS";
        
        match _data.write_slice(s.as_bytes()) {
            Ok(_) => Ok(0),
            Err(_) => 
            {
                pr_info!("read Error\n");
                Err(Error::EINVAL)
            }
        }
    }

    
    fn write(&self, _data: &mut UserSlicePtrReader, _offset: u64) -> KernelResult<usize> {
        pr_info!("Rust character device sample (write)\n");
        Ok(0)
    }
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl KernelModule for RustChrdev {
    fn init() -> KernelResult<Self> {
        pr_info!("Rust character device sample (init)\n");

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(cstr!("rust_chrdev_learning"), 0, &THIS_MODULE)?;

        // Register the same kind of device twice, we're just demonstrating
        // that you can use multiple minors. There are two minors in this case
        // because its type is `chrdev::Registration<2>`
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustChrdev { _dev: chrdev_reg })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust character device sample (exit)\n");
    }
}
