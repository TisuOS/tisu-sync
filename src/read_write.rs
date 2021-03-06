//! # 读写锁
//! 
//! 2021年4月14日

#![allow(dead_code)]

use crate::spin_mutex::SpinMutex;

/// ## 读写锁
/// 允许多个读取，一个写入
pub struct ReadWriteMutex{
    mutex : SpinMutex,
    read_cnt : usize,
    write : bool,
    core : bool,
}

impl ReadWriteMutex{
    /// core = true 意味着在临界区内禁用中断
    pub const fn new(core : bool)->Self{
        Self{
            mutex : SpinMutex::new(),
            read_cnt : 0,
            write : false,
            core
        }
    }

    pub fn read(&mut self) {
        while !self.lock_read(){}
    }

    pub fn write(&mut self) {
        while !self.lock_write(){}
    }

    pub fn unlock(&mut self){
        self.mutex.lock();
        if self.write{
            self.write = false;
        }
        else{
            self.read_cnt -= 1;
        }
        self.mutex.unlock();
    }

    fn lock_read(&mut self) ->bool {
        if self.core {
            self.mutex.lock_no_int();
        }
        else {
            self.mutex.lock();
        }
        let rt = !self.write;
        if rt {
            self.read_cnt += 1;
        }
        if self.core {
            self.mutex.unlock_no_int();
        }
        else {
            self.mutex.unlock();
        }
        rt
    }

    fn lock_write(&mut self)->bool{
        if self.core {
            self.mutex.lock_no_int();
        }
        else {
            self.mutex.lock();
        }
        let rt = self.read_cnt == 0 && !self.write;
        if rt {
            self.write = true;
        }
        if self.core {
            self.mutex.unlock_no_int();
        }
        else {
            self.mutex.unlock();
        }
        rt
    }

}
