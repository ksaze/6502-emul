use crate::bus::*;
use crate::cpu::CPU;
use crate::operations::*;

pub struct Emulator {
    pub cpu: CPU,
    pub bus: MemoryBus,
    pub instr_table: [&'static Instruction; 256],
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            bus: MemoryBus::new(),
            instr_table: [&NOP_INSTR; 256],
        }
    }

    pub fn reset_cpu(&mut self) {
        self.cpu.instr = &RESET;
        self.cpu.step = 0;
        self.cpu.ready = false;
        while !self.cpu.ready {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        if self.cpu.ready {
            let opcode = self.bus.read(self.cpu.pc);
            self.cpu.pc = self.cpu.pc.wrapping_add(1);
            self.cpu.ir = opcode;
            self.cpu.instr = self.instr_table[opcode as usize];
            self.cpu.step = 0;
            self.cpu.ready = false;
            return;
        }

        if self.cpu.step < self.cpu.instr.micro.len() {
            match (self.cpu.instr.micro[self.cpu.step])(self) {
                StepCtl::Next => {
                    self.cpu.step += 1;
                }
                StepCtl::End => {
                    self.cpu.ready = true;
                }
                StepCtl::Jump(n) => {
                    self.cpu.step = n;
                }
            }

            if self.cpu.step >= self.cpu.instr.micro.len() {
                self.cpu.ready = true;
            }
        }
    }
}
