//! # 原子值
//! 原子改变内部值
//!
//! 2021年4月19日 zg

use crate::SpinMutex;

pub struct AtomValue<T:Clone + Copy> {
    value : T,
    mutex : SpinMutex,
}

impl<T> AtomValue<T> where T: Clone + Copy {
    pub fn new(value : T)->Self {
        Self {
            value,
            mutex : SpinMutex::new(),
        }
    }

    pub fn set(&mut self, value : T) {
        self.mutex.lock();
        self.value = value;
        self.mutex.unlock();
    }

    pub fn get(&self)->T {
        self.value
    }
}
