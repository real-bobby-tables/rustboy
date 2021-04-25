#[derive(Debug)]


struct Regs {
	pc: u16;
	af: u16;
	bc: u16;
	de: u16;
	hl: u16;
	sp: u16;
}

struct CPU {
	regs: Regs
}