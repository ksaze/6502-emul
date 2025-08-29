use crate::operations::{Instruction, NOP_INSTR};
use crate::shared::*;

pub struct StackPointer(pub Byte);

impl StackPointer {
    #[must_use]
    pub const fn to_word(&self) -> Word {
        let sp_value = self.0;
        Word::from_le_bytes([sp_value, 0x01])
    }

    pub const fn decrement(&mut self) {
        self.0 = self.0.wrapping_sub(1);
    }

    pub const fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
}

pub struct CPU {
    pub pc: Word,
    pub sp: StackPointer,

    a: Byte,
    x: Byte,
    y: Byte,
    pub flags: Byte,

    pub ir: Byte,
    pub tmp8: Byte,
    pub tmp16: Word,
    pub eff: Word,
    pub crossed: bool,

    pub instr: &'static Instruction,
    pub step: usize,
    pub ready: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            sp: StackPointer(0),
            a: 0,
            x: 0,
            y: 0,
            flags: (1 << IRQ_DISABLE_FLAG_POS) | (1 << UNUSED_BIT_POS),

            ir: 0,
            tmp8: 0,
            tmp16: 0,
            eff: 0,
            crossed: false,

            instr: &NOP_INSTR,
            step: 0,
            ready: true,
        }
    }

    pub fn set_flag_bit(&mut self, pos: u8) {
        self.flags |= 1 << pos;
    }

    pub fn clear_flag_bit(&mut self, pos: u8) {
        self.flags &= 1 << pos;
    }
}
