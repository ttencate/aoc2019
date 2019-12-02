#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    pub mem: Vec<usize>,
    pub ip: usize,
}

impl Program {
    pub fn parse(input: &str) -> Self {
        Program {
            mem: input.trim().split(",").map(|s| s.parse::<usize>().unwrap()).collect(),
            ip: 0,
        }
    }

    pub fn run(mut self) -> Self {
        loop {
            let (instr, next_ip) = Instr::parse(&self.mem, self.ip);
            if !instr.exec(&mut self.mem) {
                break;
            }
            self.ip = next_ip;
        }
        self
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instr {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt(),
}

impl Instr {
    pub fn parse(mem: &Vec<usize>, ip: usize) -> (Instr, usize) {
        let opcode = mem[ip];
        match opcode {
            1 => (Instr::Add(mem[ip + 1], mem[ip + 2], mem[ip + 3]), ip + 4),
            2 => (Instr::Mul(mem[ip + 1], mem[ip + 2], mem[ip + 3]), ip + 4),
            99 => (Instr::Halt(), ip + 1),
            _ => { panic!("Invalid opcode {} at address {}", opcode, ip); }
        }
    }

    pub fn exec(&self, mem: &mut Vec<usize>) -> bool {
        match *self {
            Instr::Add(a_addr, b_addr, dest_addr) => {
                mem[dest_addr] = mem[a_addr] + mem[b_addr];
            },
            Instr::Mul(a_addr, b_addr, dest_addr) => {
                mem[dest_addr] = mem[a_addr] * mem[b_addr];
            },
            Instr::Halt() => {
                return false;
            }
        }
        true
    }
}

impl std::fmt::Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Instr::Add(a_addr, b_addr, dest_addr) =>
                write!(f, "ADD {:3} {:3} {:3}", a_addr, b_addr, dest_addr),
            Instr::Mul(a_addr, b_addr, dest_addr) =>
                write!(f, "MUL {:3} {:3} {:3}", a_addr, b_addr, dest_addr),
            Instr::Halt() =>
                write!(f, "HALT"),
        }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut ip = 0;
        while ip < self.mem.len() {
            let (instr, next_ip) = Instr::parse(&self.mem, ip);
            write!(f, "{:2}:  {}\n", ip, instr)?;
            ip = next_ip;
        }
        Ok(())
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
