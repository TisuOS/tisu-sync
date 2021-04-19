//! # 内容锁
//! 此锁免去解锁操作
//! 
//! 2021年4月3日 zg

#![allow(dead_code)]
use core::ops::{Deref, DerefMut};

use super::spin_mutex::SpinMutex;

/// ```rust
/// let mut mutex = ContentMutex::new(5);
/// {
///     let ctx = mutex.lock();
///     *ctx = 2;
/// }
/// ```
/// ## 内容锁
/// 借用临时变量进行解锁操作，减少代码量，确保百分百解锁
pub struct ContentMutex<T> {
    pub value : T,
    pub mutex : SpinMutex,
}

impl<T> ContentMutex<T> {
    pub const fn new(value : T)->Self {
        Self {
            value,
            mutex : SpinMutex::new(),
        }
    }

    pub fn raw_lock(&mut self) {
        self.mutex.lock();
    }

    pub fn unlock(&mut self) {
        self.mutex.unlock();
    }

    pub fn lock(&self)->Content<T> {
        unsafe {
            let t = self as *const Self as *mut Self;
            (*t).mutex.lock();
            Content::new(&mut *t)
        }
    }
}

impl<T:Clone> Clone for ContentMutex<T> {
    fn clone(&self) -> Self {
        Self {
            value : self.value.clone(),
            mutex: SpinMutex::new(),
        }
    }
}

pub struct Content<'a, T> {
    mutex : &'a mut ContentMutex<T>,
}

impl<'a, T> Content<'a, T> {
    pub fn new(mutex : &'a mut ContentMutex<T>)->Self {
        Self {
            mutex,
        }
    }
}

impl<T> Deref for Content<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.mutex.value
    }
}

impl<T> DerefMut for Content<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mutex.value
    }
}

impl<'a, T> Drop for Content<'a, T>{
    fn drop(&mut self) {
        self.mutex.unlock()
    }
}