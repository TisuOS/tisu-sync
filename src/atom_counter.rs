//! # 原子计数器
//! 
//! 2021年4月19日 zg

use crate::SpinMutex;

pub struct AtomCounter {
    cnt : usize,
    mutex : SpinMutex,
}

impl AtomCounter {
    pub const fn new()->Self {
        Self {
            cnt : 0,
            mutex : SpinMutex::new()
        }
    }

    /// 自增 1 并返回原来的值
    pub fn add(&mut self)->usize {
        self.mutex.lock();
        let rt = self.cnt;
        self.cnt = self.cnt.wrapping_add(1);
        self.mutex.unlock();
        rt
    }

    pub fn get(&self)->usize {
        self.mutex.lock();
        let rt = self.cnt;
        self.mutex.unlock();
        rt
    }
}