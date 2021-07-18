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
        let s1: [u8;3] = ['a' as u8, 'b' as u8, 'c' as u8];

        match _data.write_slice(&s1) {
            Ok(_) => 
            {
                pr_info!("write data\n");
                Ok(1)
            },
            Err(ret) => 
            {
                pr_info!("read Error\n");
                Err(ret)
            }
        }
    }

    
    fn write(&self, _data: &mut UserSlicePtrReader, _offset: u64) -> KernelResult<usize> {
        let mut s1: [u8;8] = [0; 8];
        let mut s2: [char;8] = [0 as char;8];
        let mut counter = 0;

        _data.read_slice(&mut s1)?;
        for c in s1.iter() {
            s2[counter] = *c as char;
            counter += 1;
        }

        let string: String = s2.iter().collect::<String>();

        pr_info!("in: {}", &string);

        pr_info!("Rust character device sample (write)\n");
        Ok(0)
    }
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<1>>>,
}

impl KernelModule for RustChrdev {
    fn init() -> KernelResult<Self> {
        pr_info!("Rust character device sample (init)\n");

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(cstr!("rust_chrdev_learning"), 0, &THIS_MODULE)?;

        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustChrdev { _dev: chrdev_reg })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust character device sample (exit)\n");
    }
}
