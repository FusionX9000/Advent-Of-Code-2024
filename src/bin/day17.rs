use advent_of_code_2024::read_input;

#[derive(Debug, Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug, Clone)]
struct Machine {
    pub registers: Registers,
    pub instructions: Vec<u64>,
    i_ptr: usize,
    pub stdout: Vec<u64>,
}

impl Machine {
    fn get_combo_operand(&self, opcode: u64) -> u64 {
        match opcode {
            n @ 0..=3 => n,
            4 => self.registers.a,
            5 => self.registers.b,
            6 => self.registers.c,
            7 => panic!("Reserved operand. Invalid program"),
            _ => panic!("Invalid operand"),
        }
    }
    fn adv(&mut self, operand: u64) {
        self.registers.a /= 1 << operand;
    }
    fn bxl(&mut self, operand: u64) {
        self.registers.b ^= operand;
    }
    fn bst(&mut self, operand: u64) {
        self.registers.b = operand % 8;
    }
    fn jnz(&mut self, operand: u64) {
        if self.registers.a != 0 {
            self.i_ptr = operand as usize;
        } else {
            self.i_ptr += 2;
        }
    }
    fn bxc(&mut self) {
        self.registers.b ^= self.registers.c
    }
    fn out(&mut self, operand: u64) {
        self.stdout.push(operand % 8)
    }
    fn bdv(&mut self, operand: u64) {
        self.registers.b = self.registers.a / (1 << operand);
    }
    fn cdv(&mut self, operand: u64) {
        self.registers.c = self.registers.a / (1 << operand);
    }
    fn execute_instruction(&mut self, opcode: u64, literal_operand: u64) {
        match opcode {
            0 => self.adv(self.get_combo_operand(literal_operand)),
            1 => self.bxl(literal_operand),
            2 => self.bst(self.get_combo_operand(literal_operand)),
            3 => self.jnz(literal_operand),
            4 => self.bxc(),
            5 => self.out(self.get_combo_operand(literal_operand)),
            6 => self.bdv(self.get_combo_operand(literal_operand)),
            7 => self.cdv(self.get_combo_operand(literal_operand)),
            _ => panic!("Invalid opcode: {opcode}"),
        };
        if opcode != 3 {
            self.i_ptr += 2;
        }
    }
    fn execute_instructions(&mut self) {
        while self.i_ptr < self.instructions.len() - 1 {
            self.execute_instruction(
                self.instructions[self.i_ptr],
                self.instructions[self.i_ptr + 1],
            );
        }
    }
    fn get_stdout(&self) -> String {
        self.stdout
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

fn parse_input(input: &str) -> Machine {
    let mut sections = input.lines().filter(|l| !l.trim().is_empty());
    let mut next_rhs = || sections.next().unwrap().split(" ").last().unwrap();
    let mut next_register = || next_rhs().parse().unwrap();
    let (a, b, c) = (next_register(), next_register(), next_register());
    Machine {
        registers: Registers { a, b, c },
        instructions: next_rhs().split(",").map(|x| x.parse().unwrap()).collect(),
        i_ptr: 0,
        stdout: Vec::new(),
    }
}

fn part1(input: &str) -> String {
    let mut machine = parse_input(input);
    machine.execute_instructions();
    machine.get_stdout()
}

fn part2(input: &str) -> String {
    String::new()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day17")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(input));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

        assert_eq!("117440", part2(input));
    }
}
