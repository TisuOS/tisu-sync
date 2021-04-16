use crate::mutex::MutexState;


/// ## 同步布尔值
/// 目前主要用于循环判断
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
    /// ### 获取并置为 unlock（false）
    pub fn pop(&mut self)->bool {
        unsafe {
            let state : MutexState;
            llvm_asm!("amoswap.w.aq $0, $1, ($2)\n" : "=r"(state) : "r"(0), "r"(self) :: "volatile");
            match state {
                MutexState::Lock => {true}
                MutexState::Unlock => {false}
            }
        }
    }

    pub fn get_val(&mut self)->bool {
        unsafe {
            let state : MutexState;
            llvm_asm!("amoor.w.aq $0, $1, ($2)" : "=r"(state) : "r"(0), "r"(self) :: "volatile");
            match state {
                MutexState::Unlock => {false}
                MutexState::Lock => {true}
            }
        }
    }
    /// ### 置为 lock（true）
    pub fn set_true(&mut self) {
        unsafe {
            let state = &mut self.state;
            llvm_asm!("amoswap.w.rl zero, $1, ($0)" :: "r"(state), "r"(1) :: "volatile");
        }
    }

    pub fn set_false(&mut self) {
        unsafe {
            let state = &mut self.state;
            llvm_asm!("amoswap.w.rl zero, $1, ($0)" :: "r"(state), "r"(0) :: "volatile");
        }
    }
}
