bitflags! {
    pub struct FlowFlags: u8 {
        const X_FORW = 0b00000001;
        const Y_FORW = 0b00000010;
        const Z_FORW = 0b00000100;
        const X_BACK = 0b00001000;
        const Y_BACK = 0b00010000;
        const Z_BACK = 0b00100000;
    }
}

impl Default for FlowFlags {
    fn default() -> Self {
        FlowFlags::empty()
    }
}
