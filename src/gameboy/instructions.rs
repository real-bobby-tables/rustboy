//https://gbdev.io/pandocs/#the-cartridge-header

enum Regs {
	A,
	B,
	C,
	AF,
	AB
}

enum Instruction {
	LoadInstruction,
	ArithmeticInstruction,
	RotateOrShiftInstruction,
	CPUControlInstruction,
	SingleBitInstruction,
	JumpInstruction,
}



enum LoadInstruction {
	ld(Regs,Regs),
	ld(String,u8),
	ld()
}