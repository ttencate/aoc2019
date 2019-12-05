use std::collections::VecDeque;

pub type Number = i64;

pub type Addr = usize;

pub fn to_addr(n: Number) -> Addr {
    n as Addr
}

pub type Memory = Vec<Number>;
pub type Input = VecDeque<Number>;
pub type Output = Vec<Number>;

pub fn parse_mem(input: &str) -> Memory {
    input.trim().split(",").map(|s| s.parse::<Number>().unwrap()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Arg {
    Position(Addr),
    Immediate(Number),
}

impl Arg {
    pub fn eval(&self, mem: &Memory) -> Number {
        match *self {
            Arg::Position(addr) => mem[addr],
            Arg::Immediate(num) => num,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    pub mem: Memory,
    pub ip: usize,
    pub input: Input,
    pub output: Output,
}

impl Program {
    pub fn new(mem: Memory) -> Self {
        Program {
            mem: mem,
            ip: 0,
            input: VecDeque::new(),
            output: vec![],
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

    fn run_instr(&mut self) -> bool {
        let mut op = self.mem[self.ip];
        self.ip += 1;
        let opcode = op % 100;
        op /= 100;
        match opcode {
            1 => { self.bin_op(op, |a, b| a + b); },
            2 => { self.bin_op(op, |a, b| a * b); },
            3 => { self.input(op); },
            4 => { self.output(op); },
            5 => { self.cond_jump(op, |a| a != 0); },
            6 => { self.cond_jump(op, |a| a == 0); },
            7 => { self.bin_op(op, |a, b| if a < b { 1 } else { 0 }); },
            8 => { self.bin_op(op, |a, b| if a == b { 1 } else { 0 }); },
            99 => { return false; }
            _ => { panic!("Invalid opcode {} at address {}", opcode, self.ip - 1); }
        }
        true
    }

    fn eval_arg(&mut self, op: &mut Number) -> Number {
        let mode = *op % 10;
        *op /= 10;
        let val = self.mem[self.ip];
        self.ip += 1;
        match mode {
            0 => self.mem[to_addr(val)],
            1 => val,
            _ => panic!("Invalid parameter mode {} at address {}", mode, self.ip - 1),
        }
    }

    fn eval_addr(&mut self) -> Addr {
        let val = self.mem[self.ip];
        self.ip += 1;
        to_addr(val)
    }

    fn bin_op<F>(&mut self, mut op: Number, f: F)
        where F: FnOnce(Number, Number) -> Number
    {
        let a = self.eval_arg(&mut op);
        let b = self.eval_arg(&mut op);
        let dest = self.eval_addr();
        self.mem[dest] = f(a, b);
    }

    fn input(&mut self, mut _op: Number) {
        let dest = self.eval_addr();
        let val = self.input.pop_front().expect("Tried to read from empty input");
        self.mem[dest] = val;
    }

    fn output(&mut self, mut op: Number) {
        let val = self.eval_arg(&mut op);
        self.output.push(val);
    }

    fn cond_jump<P>(&mut self, mut op: Number, pred: P)
        where P: FnOnce(Number) -> bool
    {
        let cond = self.eval_arg(&mut op);
        let dest = self.eval_arg(&mut op);
        if pred(cond) {
            self.ip = to_addr(dest);
        }
    }
}

#[test]
fn test_add_mul() {
    assert_eq!(
        Program::new(parse_mem("1,9,10,3,2,3,11,0,99,30,40,50")).run().mem,
        parse_mem("3500,9,10,70,2,3,11,0,99,30,40,50"));
    assert_eq!(
        Program::new(parse_mem("1,0,0,0,99")).run().mem,
        parse_mem("2,0,0,0,99"));
    assert_eq!(
        Program::new(parse_mem("2,3,0,3,99")).run().mem,
        parse_mem("2,3,0,6,99"));
    assert_eq!(
        Program::new(parse_mem("2,4,4,5,99,0")).run().mem,
        parse_mem("2,4,4,5,99,9801"));
    assert_eq!(
        Program::new(parse_mem("1,1,1,4,99,5,6,0,99")).run().mem,
        parse_mem("30,1,1,4,2,5,6,0,99"));
}

#[test]
fn test_input_output() {
    assert_eq!(
        Program::new(parse_mem("3,0,4,0,99")).with_input(vec![42]).run().output,
        vec![42]);
}

#[test]
fn test_immediate_mode() {
    assert_eq!(
        Program::new(parse_mem("1002,4,3,4,33")).run().mem,
        parse_mem("1002,4,3,4,99"));
}

#[test]
fn test_negative() {
    assert_eq!(
        Program::new(parse_mem("1101,100,-1,4,0")).run().mem,
        parse_mem("1101,100,-1,4,99"));
}

#[test]
fn test_comparisons() {
    assert_eq!(
        Program::new(parse_mem("3,9,8,9,10,9,4,9,99,-1,8")).with_input(vec![8]).run().output,
        vec![1]);
    assert_eq!(
        Program::new(parse_mem("3,9,8,9,10,9,4,9,99,-1,8")).with_input(vec![7]).run().output,
        vec![0]);

    assert_eq!(
        Program::new(parse_mem("3,9,7,9,10,9,4,9,99,-1,8")).with_input(vec![7]).run().output,
        vec![1]);
    assert_eq!(
        Program::new(parse_mem("3,9,7,9,10,9,4,9,99,-1,8")).with_input(vec![8]).run().output,
        vec![0]);

    assert_eq!(
        Program::new(parse_mem("3,3,1108,-1,8,3,4,3,99")).with_input(vec![8]).run().output,
        vec![1]);
    assert_eq!(
        Program::new(parse_mem("3,3,1108,-1,8,3,4,3,99")).with_input(vec![7]).run().output,
        vec![0]);

    assert_eq!(
        Program::new(parse_mem("3,3,1107,-1,8,3,4,3,99")).with_input(vec![7]).run().output,
        vec![1]);
    assert_eq!(
        Program::new(parse_mem("3,3,1107,-1,8,3,4,3,99")).with_input(vec![8]).run().output,
        vec![0]);
}

#[test]
fn test_jumps() {
    assert_eq!(
        Program::new(parse_mem("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")).with_input(vec![0]).run().output,
        vec![0]);
    assert_eq!(
        Program::new(parse_mem("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")).with_input(vec![8]).run().output,
        vec![1]);

    assert_eq!(
        Program::new(parse_mem("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")).with_input(vec![0]).run().output,
        vec![0]);
    assert_eq!(
        Program::new(parse_mem("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")).with_input(vec![8]).run().output,
        vec![1]);

    assert_eq!(
        Program::new(parse_mem("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99")).with_input(vec![7]).run().output,
        vec![999]);
    assert_eq!(
        Program::new(parse_mem("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99")).with_input(vec![8]).run().output,
        vec![1000]);
    assert_eq!(
        Program::new(parse_mem("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99")).with_input(vec![9]).run().output,
        vec![1001]);
}
