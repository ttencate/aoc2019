use std::collections::{HashMap, VecDeque};

pub type Number = i64;

pub type Addr = usize;

pub fn to_addr(n: Number) -> Addr {
    n as Addr
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    low: Vec<Number>,
    high: HashMap<usize, Number>,
}

impl Memory {
    pub fn parse(input: &str) -> Self {
        Memory {
            low: input.trim().split(",").map(|s| s.parse::<Number>().unwrap()).collect(),
            high: HashMap::new(),
        }
    }
}

impl std::ops::Index<usize> for Memory {
    type Output = Number;
    fn index(&self, index: usize) -> &Self::Output {
        if index < self.low.len() {
            &self.low[index]
        } else {
            self.high.get(&index).unwrap_or(&0)
        }
    }
}

impl std::ops::IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < self.low.len() {
            &mut self.low[index]
        } else {
            self.high.entry(index).or_default()
        }
    }
}

pub type Input = VecDeque<Number>;
pub type Output = VecDeque<Number>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    pub mem: Memory,
    ip: usize,
    next_ip: usize,
    relative_base: Number,
    cur_op: Number,
    pub input: Input,
    pub output: Output,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ArgMode {
    Position,
    Immediate,
    Relative,
}

impl Program {
    pub fn new(mem: Memory) -> Self {
        Program {
            mem: mem,
            ip: 0,
            next_ip: 0,
            relative_base: 0,
            cur_op: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn with_input(mut self, input: Vec<Number>) -> Self {
        self.input = input.into();
        self
    }

    pub fn run(mut self) -> Self {
        while self.run_instr() {}
        self
    }

    pub fn run_until_output(&mut self) -> Option<Number> {
        loop {
            if !self.output.is_empty() {
                return self.output.pop_front();
            }
            if !self.run_instr() {
                return None;
            }
        }
    }

    fn run_instr(&mut self) -> bool {
        self.cur_op = self.mem[self.ip];
        self.next_ip = self.ip + 1;
        let opcode = self.cur_op % 100;
        self.cur_op /= 100;
        match opcode {
            1 => { self.bin_op(|a, b| a + b); },
            2 => { self.bin_op(|a, b| a * b); },
            3 => { self.input(); },
            4 => { self.output(); },
            5 => { self.cond_jump(|a| a != 0); },
            6 => { self.cond_jump(|a| a == 0); },
            7 => { self.bin_op(|a, b| if a < b { 1 } else { 0 }); },
            8 => { self.bin_op(|a, b| if a == b { 1 } else { 0 }); },
            9 => { self.rel_base(); },
            99 => { return false; }
            _ => { panic!("Invalid opcode {} at address {}", opcode, self.ip); }
        }
        self.ip = self.next_ip;
        true
    }

    fn arg_mode(&mut self) -> ArgMode {
        let mode = self.cur_op % 10;
        self.cur_op /= 10;
        match mode {
            0 => ArgMode::Position,
            1 => ArgMode::Immediate,
            2 => ArgMode::Relative,
            _ => panic!("Invalid parameter mode {:?} at address {}", mode, self.ip),
        }
    }

    fn arg(&mut self) -> Number {
        let val = self.mem[self.next_ip];
        self.next_ip += 1;
        val
    }

    fn eval_arg(&mut self) -> Number {
        let val = self.arg();
        let mode = self.arg_mode();
        match mode {
            ArgMode::Position => self.mem[to_addr(val)],
            ArgMode::Immediate => val,
            ArgMode::Relative => self.mem[to_addr(val + self.relative_base)],
        }
    }

    fn eval_addr(&mut self) -> Addr {
        let val = self.arg();
        let mode = self.arg_mode();
        match mode {
            ArgMode::Position => to_addr(val),
            ArgMode::Relative => to_addr(val + self.relative_base),
            _ => panic!("Parameter mode {:?} at address {} not supported for lvalues", mode, self.ip)
        }
    }

    fn bin_op<F>(&mut self, f: F)
        where F: FnOnce(Number, Number) -> Number
    {
        let a = self.eval_arg();
        let b = self.eval_arg();
        let dest = self.eval_addr();
        self.mem[dest] = f(a, b);
    }

    fn input(&mut self) {
        let dest = self.eval_addr();
        let val = self.input.pop_front().expect("Tried to read from empty input");
        self.mem[dest] = val;
    }

    fn output(&mut self) {
        let val = self.eval_arg();
        self.output.push_back(val);
    }

    fn cond_jump<P>(&mut self, pred: P)
        where P: FnOnce(Number) -> bool
    {
        let cond = self.eval_arg();
        let dest = self.eval_arg();
        if pred(cond) {
            self.next_ip = to_addr(dest);
        }
    }

    fn rel_base(&mut self) {
        let delta = self.eval_arg();
        self.relative_base += delta;
    }
}

#[test]
fn test_add_mul() {
    assert_eq!(
        Program::new(Memory::parse("1,9,10,3,2,3,11,0,99,30,40,50")).run().mem,
        Memory::parse("3500,9,10,70,2,3,11,0,99,30,40,50"));
    assert_eq!(
        Program::new(Memory::parse("1,0,0,0,99")).run().mem,
        Memory::parse("2,0,0,0,99"));
    assert_eq!(
        Program::new(Memory::parse("2,3,0,3,99")).run().mem,
        Memory::parse("2,3,0,6,99"));
    assert_eq!(
        Program::new(Memory::parse("2,4,4,5,99,0")).run().mem,
        Memory::parse("2,4,4,5,99,9801"));
    assert_eq!(
        Program::new(Memory::parse("1,1,1,4,99,5,6,0,99")).run().mem,
        Memory::parse("30,1,1,4,2,5,6,0,99"));
}

#[test]
fn test_input_output() {
    assert_eq!(
        Program::new(Memory::parse("3,0,4,0,99")).with_input(vec![42]).run().output,
        vec![42]);
}

#[test]
fn test_immediate_mode() {
    assert_eq!(
        Program::new(Memory::parse("1002,4,3,4,33")).run().mem,
        Memory::parse("1002,4,3,4,99"));
}

#[test]
fn test_negative() {
    assert_eq!(
        Program::new(Memory::parse("1101,100,-1,4,0")).run().mem,
        Memory::parse("1101,100,-1,4,99"));
}

#[test]
fn test_comparisons() {
    assert_eq!(
        Program::new(Memory::parse("3,9,8,9,10,9,4,9,99,-1,8")).with_input(vec![8]).run().output,
        vec![1]);
    assert_eq!(
        Program::new(Memory::parse("3,9,8,9,10,9,4,9,99,-1,8")).with_input(vec![7]).run().output,
        vec![0]);

    assert_eq!(
        Program::new(Memory::parse("3,9,7,9,10,9,4,9,99,-1,8")).with_input(vec![7]).run().output,
        vec![1]);
    assert_eq!(
        Program::new(Memory::parse("3,9,7,9,10,9,4,9,99,-1,8")).with_input(vec![8]).run().output,
        vec![0]);

    assert_eq!(
        Program::new(Memory::parse("3,3,1108,-1,8,3,4,3,99")).with_input(vec![8]).run().output,
        vec![1]);
    assert_eq!(
        Program::new(Memory::parse("3,3,1108,-1,8,3,4,3,99")).with_input(vec![7]).run().output,
        vec![0]);

    assert_eq!(
        Program::new(Memory::parse("3,3,1107,-1,8,3,4,3,99")).with_input(vec![7]).run().output,
        vec![1]);
    assert_eq!(
        Program::new(Memory::parse("3,3,1107,-1,8,3,4,3,99")).with_input(vec![8]).run().output,
        vec![0]);
}

#[test]
fn test_jumps() {
    assert_eq!(
        Program::new(Memory::parse("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")).with_input(vec![0]).run().output,
        vec![0]);
    assert_eq!(
        Program::new(Memory::parse("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")).with_input(vec![8]).run().output,
        vec![1]);

    assert_eq!(
        Program::new(Memory::parse("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")).with_input(vec![0]).run().output,
        vec![0]);
    assert_eq!(
        Program::new(Memory::parse("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")).with_input(vec![8]).run().output,
        vec![1]);

    assert_eq!(
        Program::new(Memory::parse("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99")).with_input(vec![7]).run().output,
        vec![999]);
    assert_eq!(
        Program::new(Memory::parse("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99")).with_input(vec![8]).run().output,
        vec![1000]);
    assert_eq!(
        Program::new(Memory::parse("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99")).with_input(vec![9]).run().output,
        vec![1001]);
}

#[test]
fn test_relative_mode() {
    assert_eq!(
        Program::new(Memory::parse("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99")).run().output,
        vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);
    assert_eq!(
        Program::new(Memory::parse("1102,34915192,34915192,7,4,7,99,0")).run().output[0].to_string().len(),
        16);
    assert_eq!(
        Program::new(Memory::parse("104,1125899906842624,99")).run().output,
        vec![1125899906842624]);
}
