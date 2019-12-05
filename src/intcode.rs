type Number = i64;

type Addr = usize;

pub fn to_addr(n: Number) -> Addr {
    n as Addr
}

type Memory = Vec<Number>;

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
}

impl Program {
    pub fn parse(input: &str) -> Self {
        Program {
            mem: input.trim().split(",").map(|s| s.parse::<Number>().unwrap()).collect(),
            ip: 0,
        }
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
}

#[test]
fn test_add_mul() {
    assert_eq!(
        Program::parse("1,9,10,3,2,3,11,0,99,30,40,50").run().mem,
        Program::parse("3500,9,10,70,2,3,11,0,99,30,40,50").mem);
    assert_eq!(
        Program::parse("1,0,0,0,99").run().mem,
        Program::parse("2,0,0,0,99").mem);
    assert_eq!(
        Program::parse("2,3,0,3,99").run().mem,
        Program::parse("2,3,0,6,99").mem);
    assert_eq!(
        Program::parse("2,4,4,5,99,0").run().mem,
        Program::parse("2,4,4,5,99,9801").mem);
    assert_eq!(
        Program::parse("1,1,1,4,99,5,6,0,99").run().mem,
        Program::parse("30,1,1,4,2,5,6,0,99").mem);
}

#[test]
fn test_immediate_mode() {
    assert_eq!(
        Program::parse("1002,4,3,4,33").run().mem,
        Program::parse("1002,4,3,4,99").mem);
}

#[test]
fn test_negative() {
    assert_eq!(
        Program::parse("1101,100,-1,4,0").run().mem,
        Program::parse("1101,100,-1,4,99").mem);
}
