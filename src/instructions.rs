pub type Register = u8;

#[allow(dead_code)]
// This is not based off a standard instruction set
pub enum Instruction
{
	NoOp,
	LoadValue(u8, Register),
	LoadFromMem(u8, Register),
	Store(Register, u8),
	Add(Register, Register),
	Sub(Register, Register),
	Jump(u8),
	And(Register, Register),
	Or(Register, Register)
}
