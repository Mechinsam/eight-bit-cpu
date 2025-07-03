use crate::instructions::Instruction;

pub struct CPU
{
	registers: Vec<u8>, // 3 registers
	memory: Vec<u8>, // 64 bytes of ram
	pc: usize
}

impl CPU
{
	pub fn new(register_count: u8, memory_size: u8) -> Self
	{
		Self {
			registers: vec![0; register_count as usize],
			memory: vec![0; memory_size as usize],
			pc: 0
		}
	}
	pub fn display_info(&self)
	{
		let reg_count: u8 = self.registers.len() as u8;
		let mem_count: u8 = self.memory.len() as u8;

		println!("CPU: 8-bit CPU Emulator@1 Core(s)@[unknown]Hz\n---\nRegister count: {}\nVirtual Memory: {} bytes\n---", reg_count, mem_count);

		for x in 0..self.registers.len()
		{
			println!("Register\t{}: {} ({:08b})", x as u8, self.registers[x], self.registers[x])
		}
	}
	pub fn expose_memory(&self)
	{
		println!("\nFULL MEMORY EXPOSURE\n---");
		for address in 0..self.memory.len()
		{
			println!("Address {}:\t{} ({:08b})", address as u8, self.memory[address], self.memory[address]);
		}
	}

	pub fn step(&mut self, instruction: Instruction)
	{
		match instruction
		{
			Instruction::NoOp =>
			{
			}
			Instruction::LoadValue(value, regidx) =>
			{
				let regidx = regidx as usize;

				self.registers[regidx] = value;
			}
			Instruction::LoadFromMem(mem_addr, reg_idx) =>
			{
				let mem_addr: usize = mem_addr as usize;
				let reg_idx: usize = reg_idx as usize;

				self.registers[reg_idx] = self.memory[mem_addr];
			}
			Instruction::Store(reg_idx, mem_addr) =>
			{
				let reg_idx: usize = reg_idx as usize;
				let mem_addr: usize = mem_addr as usize;

				self.memory[mem_addr] = self.registers[reg_idx];
			}
			Instruction::Add(reg_idx1, reg_idx2) =>
			{
				let reg_idx1: usize = reg_idx1 as usize;
				let reg_idx2: usize = reg_idx2 as usize;

				self.registers[reg_idx1] = self.registers[reg_idx1] + self.registers[reg_idx2];
			}
			Instruction::Sub(reg_idx1, reg_idx2) =>
			{
				let reg_idx1: usize = reg_idx1 as usize;
				let reg_idx2: usize = reg_idx2 as usize;

				self.registers[reg_idx1] = self.registers[reg_idx1] - self.registers[reg_idx2];
			}
			Instruction::Jump(mem_addr) =>
			{
				let mem_addr: usize = mem_addr as usize;

				self.pc = mem_addr;
				return;
			}
			Instruction::And(reg_idx1, reg_idx2) =>
			{
				let reg_idx1: usize = reg_idx1 as usize;
				let reg_idx2: usize = reg_idx2 as usize;

				self.registers[reg_idx1] = self.registers[reg_idx1] & self.registers[reg_idx2];
			}
			Instruction::Or(reg_idx1, reg_idx2) =>
			{
				let reg_idx1: usize = reg_idx1 as usize;
				let reg_idx2: usize = reg_idx2 as usize;

				self.registers[reg_idx1] = self.registers[reg_idx1] | self.registers[reg_idx2];
			}
		}

		self.pc = (self.pc + 1) & 0x0F;
	}
}
