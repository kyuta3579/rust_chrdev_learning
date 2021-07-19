#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{
    chrdev, 
    c_str,
    str::CStr, 
    file_operations::FileOperations,
    file::File,
    io_buffer::{IoBufferReader, IoBufferWriter},
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

    fn read<T: IoBufferWriter>(_this: &Self, _file: &File, _data: &mut T, _offset: u64,) -> Result<usize> {
        pr_info!("Rust character device: read\n");
        let ret_str = c_str!("read_fp: testing");

        match _data.write_slice(ret_str.as_bytes()) {
            Ok(_) => 
            {
                pr_info!("write data\n");
                Ok(0)
            },
            Err(ret) => 
            {
                pr_info!("write Error\n");
                Err(ret)
            }
        }
    }

    fn write<T: IoBufferReader>(_this: &Self, _file: &File, _data: &mut T, _offset: u64,) -> Result<usize> {
        let mut input_str: [u8;8] = [0; 8];

        _data.read_slice(&mut input_str)?;
 
        let conv_str = CStr::from_bytes_with_nul_unwrap(&input_str);

        pr_info!("input: {}\n", *conv_str.as_char_ptr());

        Ok(0)
    }
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl KernelModule for RustChrdev {
    fn init() -> Result<Self> {
        pr_info!("Rust character device sample (init)\n");

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(c_str!("rust_chrdev_learning"), 0, &THIS_MODULE)?;

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
