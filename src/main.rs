mod instructions;
mod cpu;

use cpu::CPU;
use instructions::Instruction;

fn main() {
	let mut new_cpu = CPU::new(4, 64);

	new_cpu.step(Instruction::LoadValue(1, 0));
	new_cpu.step(Instruction::LoadValue(1, 1));
	new_cpu.step(Instruction::Add(0, 1));

	new_cpu.display_info();
	new_cpu.expose_memory();
}
