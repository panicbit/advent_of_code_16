extern crate regex;
use regex::Regex;
use std::ops;

fn main() {
    let input = include_str!("../input.txt");
    let code: Vec<Op> = input.lines().map(|line| Op::from_str(line.trim())).collect();
    let mut cpu = Cpu::new();
    cpu.run(&code);

    println!("{:?}", cpu.a);
}

struct Cpu {
    ip: usize,
    a: RegType,
    b: RegType,
    c: RegType,
    d: RegType,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            ip: 0,
            a: 0,
            b: 0,
            c: 1,
            d: 0,
        }
    }
    fn run(&mut self, code: &[Op]) {
        while self.step(code) {}
    }
    fn step(&mut self, code: &[Op]) -> bool {
        if self.ip >= code.len() {
            return false;
        }

        let old_ip = self.ip;
        self.ip += 1;

        match code[old_ip] {
            Op::Inc(r) => self[r] += 1,
            Op::Dec(r) => self[r] -= 1,
            Op::Cpy(Source::Imm(v), r) => self[r] = v,
            Op::Cpy(Source::Reg(r1), r2) => self[r2] = self[r1],
            Op::Jnz(Source::Imm(0), _) => {},
            Op::Jnz(Source::Reg(r), _) if self[r] == 0 => {},
            Op::Jnz(_, by) => self.ip = (old_ip as isize + by as isize) as usize,
        }

        old_ip != self.ip
    }
}

impl ops::Index<Reg> for Cpu {
    type Output = RegType;
    fn index(&self, r: Reg) -> &Self::Output {
        match r {
            Reg::A => &self.a,
            Reg::B => &self.b,
            Reg::C => &self.c,
            Reg::D => &self.d,
        }
    }
}

impl ops::IndexMut<Reg> for Cpu {
    fn index_mut(&mut self, r: Reg) -> &mut Self::Output {
        match r {
            Reg::A => &mut self.a,
            Reg::B => &mut self.b,
            Reg::C => &mut self.c,
            Reg::D => &mut self.d,
        }
    }
}

#[derive(Debug,Copy,Clone)]
enum Op {
    Cpy(Source,Reg),
    Inc(Reg),
    Dec(Reg),
    Jnz(Source,Imm),
}

impl Op {
    fn from_str(s: &str) -> Self {
        let re = Regex::new(r"([^ ]+) (.+)").unwrap();
        let c = re.captures(s).unwrap();
        let op = &c[1];
        let args = &c[2];
        match op {
            "inc" => Op::Inc(Reg::from_str(args)),
            "dec" => Op::Dec(Reg::from_str(args)),
            "jnz" => {
                let re = Regex::new(r"([^ ]+) (.+)").unwrap();
                let c = re.captures(args).unwrap();
                Op::Jnz(Source::from_str(&c[1]), c[2].parse().unwrap())
            }
            "cpy" => {
                let re = Regex::new(r"([^ ]+) (.+)").unwrap();
                let c = re.captures(args).unwrap();
                Op::Cpy(Source::from_str(&c[1]), Reg::from_str(&c[2]))
            }
            _ => panic!("Unknown op: {}", op)
        }
    }
}

type RegType = i32;

#[derive(Debug,Copy,Clone)]
enum Reg {
    A, B, C, D
}

impl Reg {
    fn from_str(s: &str) -> Self {
        match s {
            "a" => Reg::A,
            "b" => Reg::B,
            "c" => Reg::C,
            "d" => Reg::D,
            _ => panic!("Unknown register: {}", s)
        }
    }
}

type Imm = RegType;

#[derive(Debug,Copy,Clone)]
enum Source {
    Imm(Imm),
    Reg(Reg),
}

impl Source {
    fn from_str(s: &str) -> Self {
        match s {
            "a" | "b" | "c" | "d" => Source::Reg(Reg::from_str(s)),
            _ => Source::Imm(s.parse().unwrap())
        }
    }
}
