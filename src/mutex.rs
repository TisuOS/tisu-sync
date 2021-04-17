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
/// ## 多重锁
/// 多重锁，允许一个核心多次上锁，这是为了解决单重锁在多重函数中反复上锁的需求,后期应当避免使用此锁
#[repr(C)]
pub struct MultiMutex {
    mutex : Mutex,
    cnt : usize,
    hartid : usize,
}

/// ## 自旋锁
/// 提供更加细致的可自定义的功能，不要将它与标准库的 Mutex 搞混
/// ```rust
/// let mut mutex = Mutex::new();
/// mutex.lock();
/// // do something
/// mutex.unlock();
/// ```
#[repr(C)]
pub struct Mutex{
    pub state : MutexState,
}

/// 上锁原理：
/// 传入上锁的 CPU 核心号，每次上锁进行计数，当锁存在且不属于当前 CPU 时，阻塞
impl MultiMutex {
    pub const fn new() -> Self {
        MultiMutex {
            mutex : Mutex::new(),
            hartid : 1000,
            cnt : 0,
        }
    }
    pub fn lock(&mut self, hartid : usize) {
        // 已经上锁且核心不同
        while !self.lock_state(hartid){}
    }
    pub fn unlock(&mut self){
        self.mutex.lock();
        self.cnt -= 1;
        if self.cnt == 0{
            self.hartid = 1000;
        }
        self.mutex.unlock();
    }
    fn lock_state(&mut self, hartid : usize) ->bool {
        self.mutex.lock();
        let rt = self.cnt == 0 || hartid == self.hartid;
        if rt {
            self.hartid = hartid;
            self.cnt += 1;
        }
        self.mutex.unlock();
        rt
    }
}

/// 通过 原子 swap 实现
impl Mutex{
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Mutex {
            state : MutexState::Unlock,
        }
    }
    pub fn lock(&mut self) {
        while !self.lock_state() {}
    }
    pub fn unlock(&mut self){
        unsafe {
            let mut addr = &mut self.state as *mut MutexState as usize;
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