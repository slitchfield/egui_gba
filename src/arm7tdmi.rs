
pub enum Status {
    SystemUser,
    _Fiq,
    _Supervisor,
    _Abort,
    _Irq,
    _Undefined
}

pub struct Arm7TDMI {
    pub status: Status,
    pub regfile: [u32; 37],
}

impl Default for Arm7TDMI {
    fn default() -> Self {
        Self {
            status: Status::SystemUser,
            regfile: [0u32; 37],
        }
    }
}

impl Arm7TDMI {

    pub fn print_state(&self) -> String {
        let mut ret_str: String = String::new();
        ret_str.push_str("Hello from print_state()");
        return ret_str
    }

    pub fn get_cpsr(&self) -> u32 {
        return self.regfile[15]
    }
}