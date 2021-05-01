//! # 同步锁
//! 同步锁，用于磁盘和进程
//! 同步锁只需要保证上锁部分的代码不会与其它进程重合，就能保证有效性
//! 所以只需要实现一个简单的互斥锁就可以完成其它锁
//! 2020年11月 zg
#![allow(unused_assignments)]
#![allow(dead_code)]

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub enum MutexState{
    Unlock = 0,
    Lock = 1,
}

impl MutexState {
    pub fn val(self)->usize{
        self as usize
    }

    pub fn from(v : usize)->Self {
        match v {
            0 => {Self::Unlock}
            1 => {Self::Lock}
            _ => {panic!("Unhandler state")}
        }
    }
}

/// ## 自旋锁
/// 提供更加细致的可自定义的功能，不要将它与标准库的 SpinMutex 搞混
/// ```rust
/// let mut mutex = SpinMutex::new();
/// mutex.lock();
/// // do something
/// mutex.unlock();
/// ```
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpinMutex{
    pub state : MutexState,
}

/// 通过 原子 swap 实现
impl SpinMutex{
    #[allow(dead_code)]
    pub const fn new() -> Self {
        SpinMutex {
            state : MutexState::Unlock,
        }
    }
    pub fn lock(&self) {
        unsafe {
            let t = self as *const Self as *mut Self;
            while !(*t).lock_state() {}
        }
    }
    pub fn unlock(&self){
        unsafe {
            let t = self as *const Self as *mut Self;
            let mut addr = &mut (*t).state as *mut MutexState as usize;
            asm!(
                "amoswap.w.rl zero, zero, ({state})",
                state = inout(reg)addr
            );
        }
    }
    fn lock_state(&mut self) ->bool {
        unsafe {
            let mut state = 0;
            let mut addr = self as *mut Self as *mut MutexState as usize;
            asm!(
                "amoswap.w.aq {state}, {v}, ({src})",
                state = out(reg)state,
                src = inout(reg)addr,
                v = in(reg) 1,
            );
            match MutexState::from(state) {
                MutexState::Lock => {false}
                MutexState::Unlock => {true}
            }
        }
    }
    pub fn sync<F>(&mut self, mut f : F) where F : FnMut() {
        self.lock();
        f();
        self.unlock();
    }
}



// use crate::uart;