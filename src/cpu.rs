const CARRY_FLAG_POS: u8 = 0;
const ZERO_FLAG_POS: u8 = 1;
const IRQ_DISABLE_FLAG_POS: u8 = 2;
const DECIMAL_FLAG_POS: u8 = 3;
const BREAK_FLAG_POS: u8 = 4;
const OVERFLOW_FLAG_POS: u8 = 6;
const NEGATIVE_FLAG_POS: u8 = 7;

struct FlagsRegister {
    carry: bool,
    zero: bool,
    irq_disable: bool,
    decimal: bool,
    brk: bool,
    overflow: bool,
    negative: bool,
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_POS
            | (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_POS
            | (if flag.irq_disable { 1 } else { 0 }) << IRQ_DISABLE_FLAG_POS
            | (if flag.decimal { 1 } else { 0 }) << DECIMAL_FLAG_POS
            | (if flag.brk { 1 } else { 0 }) << BREAK_FLAG_POS
            | (if flag.overflow { 1 } else { 0 }) << OVERFLOW_FLAG_POS
            | (if flag.negative { 1 } else { 0 }) << NEGATIVE_FLAG_POS
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let carry = ((byte >> CARRY_FLAG_POS) & 0b1) != 0;
        let zero = ((byte >> ZERO_FLAG_POS) & 0b1) != 0;
        let irq_disable = ((byte >> IRQ_DISABLE_FLAG_POS) & 0b1) != 0;
        let decimal = ((byte >> DECIMAL_FLAG_POS) & 0b1) != 0;
        let brk = ((byte >> BREAK_FLAG_POS) & 0b1) != 0;
        let overflow = ((byte >> OVERFLOW_FLAG_POS) & 0b1) != 0;
        let negative = ((byte >> NEGATIVE_FLAG_POS) & 0b1) != 0;

        FlagsRegister {
            carry,
            zero,
            irq_disable,
            decimal,
            brk,
            overflow,
            negative,
        }
    }
}

pub struct CPU {
    pc: u16,
    sp: u8,

    a: u8,
    x: u8,
    y: u8,
    flags: u8,

    memory: [u8; 0x10000],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0xFFFC,
            sp: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            flags: 1 << IRQ_DISABLE_FLAG_POS,
            memory: [0; 0x10000],
        }
    }

    pub fn reset_cpu(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFF;
        self.pc = 0xFFFC;
        self.flags = 1 << IRQ_DISABLE_FLAG_POS;
        self.memory.fill(0);
    }
}
