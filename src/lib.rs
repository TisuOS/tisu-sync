//! # 同步锁库
//! 提供多种锁，包括类似Rust标准库的ContentMutex，以及更简单的Mutex自旋锁
//! 
//! 2021年4月16日 zg

#![no_std]
#![feature(
    asm
)]

mod spin_mutex;
mod read_write;
mod content_mutex;
mod mutex_bool;
mod atom_value;
mod atom_counter;

pub use atom_counter::AtomCounter;
pub use atom_value::AtomValue;
pub use spin_mutex::SpinMutex;
pub use content_mutex::ContentMutex;
pub use read_write::ReadWriteMutex;
pub use mutex_bool::Bool;
