//! # 同步锁库
//! 提供多种锁，包括类似Rust标准库的ContentMutex，以及更简单的Mutex自旋锁
//! 
//! 2021年4月16日 zg

#![no_std]
#![feature(
    asm
)]

mod mutex;
mod read_write;
mod content;
mod mutex_bool;

pub use mutex::Mutex;
pub use content::ContentMutex;
pub use read_write::ReadWriteMutex;
pub use mutex_bool::Bool;
