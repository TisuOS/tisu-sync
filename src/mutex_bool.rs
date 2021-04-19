#![allow(unused_assignments)]
use crate::spin_mutex::MutexState;


/// ## 同步布尔值
/// 确保内部布尔值原子操作
/// ```rust
/// let mut b = Bool::new();
/// b.set(false);
/// ```
pub struct Bool {
    state : MutexState,
}

impl Bool {
    /// ### 初始化为 unlock，对应 false
    pub const fn new()->Self {
        Self {
            state : MutexState::Unlock,
        }
    }

    pub fn set(&mut self, val : bool) {
        match val {
            true => self.set_true(),
            false => self.set_false(),
        }
    }

    /// ### 获取并置为 unlock（false）
    pub fn pop(&mut self)->bool {
        unsafe {
            let mut state = 0;
            let mut addr = self as *mut Self as usize;
            asm!(
                "amoswap.w.aq {rev}, {rd}, ({rs})",
                rev = out(reg) state,
                rd = in(reg) state,
                rs = inout(reg) addr
            );
            match MutexState::from(state) {
                MutexState::Lock => {true}
                MutexState::Unlock => {false}
            }
        }
    }

    pub fn get_val(&mut self)->bool {
        unsafe {
            let mut state = 0;
            let mut addr = self as *mut Self as usize;
            asm!(
                "amoor.w.aq {rt}, zero, ({val})",
                rt = out(reg)state,
                val = inout(reg)addr
            );
            match MutexState::from(state) {
                MutexState::Unlock => {false}
                MutexState::Lock => {true}
            }
        }
    }
    /// ### 置为 lock（true）
    pub fn set_true(&mut self) {
        unsafe {
            let mut addr = &mut self.state as *mut MutexState as usize;
            asm!(
                "amoswap.w.rl zero, {v}, ({state})",
                state = inout(reg)addr,
                v = in(reg) 1,
            );
        }
    }

    pub fn set_false(&mut self) {
        unsafe {
            let state = &mut self.state;
            let mut addr = state as *mut MutexState as usize;
            asm!(
                "amoswap.w.rl zero, {v}, ({state})",
                state = inout(reg)addr,
                v = in(reg)0
            );
        }
    }
}
