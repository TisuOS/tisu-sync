//! # 同步锁
//! 同步锁，用于磁盘和进程
//! 同步锁只需要保证上锁部分的代码不会与其它进程重合，就能保证有效性
//! 所以只需要实现一个简单的互斥锁就可以完成其它锁
//! 2020年11月 zg

#![allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub enum MutexState{
    Unlock = 0,
    Lock = 1,
}
/// ## 多重锁
/// 多重锁，允许一个核心多次上锁，这是为了解决单重锁在多重函数中反复上锁的需求,后期应当避免使用此锁
#[repr(C)]
pub struct MultiMutex {
    mutex : Mutex,
    cnt : usize,
    hartid : usize,
}

/// ## 单重锁
/// 单次锁，仅仅针对简单的同步要求
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
            let state = &mut self.state;
            llvm_asm!("amoswap.w.rl zero, zero, ($0)" :: "r"(state) :: "volatile");
        }
    }
    fn lock_state(&mut self) ->bool {
        unsafe {
            let state : MutexState;
            llvm_asm!("amoswap.w.aq $0, $1, ($2)\n" : "=r"(state) : "r"(1), "r"(self) :: "volatile");
            match state {
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