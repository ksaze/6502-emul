use crate::bus::*;
use crate::cpu::StackPointer;
use crate::emulator::Emulator;
use crate::shared::{DECIMAL_FLAG_POS, IRQ_DISABLE_FLAG_POS, UNUSED_BIT_POS, Word};

type MicroOp = fn(emul: &mut Emulator) -> StepCtl;

#[derive(Copy, Clone)]
pub enum StepCtl {
    Next,
    End,
    Jump(usize), // jump to micro-op index (for conditional extra cycles)
}

pub struct Instruction {
    pub name: &'static str,
    pub micro: &'static [MicroOp],
}

static DUMMY_READ: MicroOp = |emu| {
    emu.bus.read(0x00FF);
    StepCtl::Next
};

pub static NOP_INSTR: Instruction = Instruction {
    name: "NOP",
    micro: &[|_emu| StepCtl::End],
};

pub static RESET: Instruction = Instruction {
    name: "RESET",
    micro: &[
        |emu| {
            emu.cpu.sp = StackPointer(0);
            emu.cpu.ir = 0;
            emu.cpu.set_flag_bit(IRQ_DISABLE_FLAG_POS);
            emu.cpu.set_flag_bit(UNUSED_BIT_POS);
            emu.cpu.clear_flag_bit(DECIMAL_FLAG_POS);
            emu.bus.read(0x00FF);
            StepCtl::Next
        },
        DUMMY_READ,
        DUMMY_READ,
        // Cycle 3: fake stack push of PCH -> actually a READ from 0x0100+SP
        |emu| {
            let addr = 0x0100u16.wrapping_add(emu.cpu.sp.to_word());
            emu.bus.read(addr); // discard
            emu.cpu.sp.decrement();
            StepCtl::Next
        },
        // Cycle 4: fake stack push of PCL
        |emu| {
            let addr = 0x0100u16.wrapping_add(emu.cpu.sp.to_word());
            emu.bus.read(addr);
            emu.cpu.sp.decrement();
            StepCtl::Next
        },
        // Cycle 5: fake stack push of P
        |emu| {
            let addr = 0x0100u16.wrapping_add(emu.cpu.sp.to_word());
            emu.bus.read(addr);
            emu.cpu.sp.decrement();
            StepCtl::Next
        },
        // Cycle 6: fetch low byte of RESET vector at $FFFC
        |emu| {
            let lo = emu.bus.read(0xFFFC);
            emu.cpu.tmp8 = lo;
            StepCtl::Next
        },
        // Cycle 7: fetch high byte of RESET vector at $FFFD
        |emu| {
            let hi = emu.bus.read(0xFFFD);
            emu.cpu.pc = Word::from_le_bytes([emu.cpu.tmp8, hi]);
            StepCtl::Next
        },
        // Cycle 8: fetch first real opcode into IR
        |emu| {
            let opcode = emu.bus.read(emu.cpu.pc);
            emu.cpu.ir = opcode;
            emu.cpu.pc = emu.cpu.pc.wrapping_add(1);
            emu.cpu.ready = true; // now we’re ready for the next real instruction
            StepCtl::End
        },
    ],
};
