struct IntComp {
    pc: i64,
    memory: Vec<i64>,
}

fn make_int_comp(data: Vec<i64>) -> IntComp {
    IntComp{
	pc: 0,
	memory: data
    }
}

impl IntComp {
    fn print(&self) {
	println!("IntComp:\n\tPC: {}\n\tMemory: {:?}", self.pc, self.memory)
    }

    fn run(&mut self) {
	while self.memory[self.pc as usize] != 99 {
	    self.run_instruction();
	    self.pc += 4
	}
    }

    fn run_instruction(&mut self) {
	let opcode = self.memory[self.pc as usize];
	match opcode {
	    1 => {
		let src1 = self.memory[(self.pc + 1) as usize] as usize;
		let src2 = self.memory[(self.pc + 2) as usize] as usize;
		let dest = self.memory[(self.pc + 3) as usize] as usize;
		self.memory[dest] = self.memory[src1] + self.memory[src2];
	    }
	    2 => {
		let src1 = self.memory[(self.pc + 1) as usize] as usize;
		let src2 = self.memory[(self.pc + 2) as usize] as usize;
		let dest = self.memory[(self.pc + 3) as usize] as usize;
		self.memory[dest] = self.memory[src1] * self.memory[src2];
	    }
	    _ => println!("Invalid opcode: {}", opcode)
	}
    }

    fn print_program(&self) {
	let mut i = 0;
	while i < self.memory.len() {
	    match self.memory[i] {
		1 =>  println!("add {} {} -> {}", self.memory[i+1], self.memory[i+2], self.memory[i+3]),
		2 =>  println!("mul {} {} -> {}", self.memory[i+1], self.memory[i+2], self.memory[i+3]),
		99 => { println!("hlt");     i+=1; continue;},
		_ =>  { println!("invalid"); i+=1; continue;}
	    }
	    i += 4;
	}
    }
}

fn main() {
    
    let data = vec![
	1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,2,19,13,23,1,23,10,27,1,13,27,31,2,31,10,35,1,35,9,39,1,39,13,43,1,13,43,47,1,47,13,51,1,13,51,55,1,5,55,59,2,10,59,63,1,9,63,67,1,6,67,71,2,71,13,75,2,75,13,79,1,79,9,83,2,83,10,87,1,9,87,91,1,6,91,95,1,95,10,99,1,99,13,103,1,13,103,107,2,13,107,111,1,111,9,115,2,115,10,119,1,119,5,123,1,123,2,127,1,127,5,0,99,2,14,0,0
    ];
    for noun in 0..100 {
	for verb in 0..100 {
	    let mut nmem = data.clone();
	    nmem[1] = noun;
	    nmem[2] = verb;
	    let mut int_comp = make_int_comp(nmem);
	    int_comp.run();
	    if int_comp.memory[0] == 19690720 {
		println!("Noun: {}\nVerb: {}", noun, verb);
		break;
	    }
	}
    }
//    let mut int_comp = make_int_comp(data);
//    int_comp.print_program();
//    int_comp.print();
//    int_comp.run();
//    int_comp.print();
}
