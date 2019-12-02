use std::fs::File;
use std::io::{self, prelude::*};

struct IntCodeCPU {
    memory: Vec<usize>,
    instruction_pointer: usize
}

impl IntCodeCPU {

    fn new() -> Self {
        IntCodeCPU {
            memory: Vec::new(),
            instruction_pointer: 0
        }
    }

    fn load_program(&mut self, program: &str){
        self.memory = program.trim().split(",")
        .map(|code|{
            code.parse::<usize>().unwrap()
        })
        .collect();
        self.instruction_pointer = 0;
    }

    fn set_parameters(&mut self, noun: usize, verb: usize){
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    fn run(&mut self) {
        loop{
            let instruction: Vec<usize> = self.memory.iter().skip(self.instruction_pointer).take(4).cloned().collect();
            match instruction.as_slice() {
                [99,_,_,_] => break,
                [1,op1,op2,res] => self.memory[*res] = self.memory[*op1] + self.memory[*op2],
                [2,op1,op2,res] => self.memory[*res] = self.memory[*op1] * self.memory[*op2],
                _ => unimplemented!()
            }
            self.instruction_pointer += 4;
        }
    }

    fn get_result(&self) -> usize {
        return self.memory[0];
    }
}


fn main()-> io::Result<()> {
    // Puzzle 1
    let mut file = File::open("input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    
    let mut machine = IntCodeCPU::new();
    machine.load_program(&buffer);
    // before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
    machine.set_parameters(12, 2);
    machine.run();    
    let result= machine.get_result();
    println!("First Puzzle: {}",result);

    // puzzle 2 
    for noun in 0..100 {
        for verb in 0..100 {
            machine.load_program(&buffer);
            machine.set_parameters(noun, verb);
            machine.run();
            let result= machine.get_result();
            if result == 19690720 {
                println!("Second Puzzle: {}", noun*100+verb);
                return Ok(());
            }
        }
    }
    Ok(())
}
