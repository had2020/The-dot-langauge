use std::f32::consts::PI;

//#[repr(u8)] we can't have this, it's too discrete, just imagine bit packing, even worse!
pub enum ANALOGOPCODES {
    MOV = 0x00, // cur address, set address
    IMM = 0x01, // set address, value
    ADD = 0x02, // Just guess the rest bro.
    SUB = 0x03,
    MUL = 0x04,
    DIV = 0x05,
    SQRT = 0x06,
    RAISE = 0x07,
    CRTSUBROUTINES = 0x08,
    CALLSUBROUTINES = 0x09,
    JMP = 0x10,
    JCC = 0x11,
    JGT = 0x12,
    JLT = 0x13,
    NOP = 0x14,
}

#[derive(Debug, Clone)]
pub struct AnalogCalculusTrueNumber {
    pub content: Vec<u8>, // And no bitpacking base 10 digits, because that's not human readable enough!
}

impl AnalogCalculusTrueNumber {
    pub fn create_new_truth() -> Self {
        AnalogCalculusTrueNumber {
            content: vec![0x00],
        }
    }

    pub fn create_digit_of_truth(digit: u8) -> Self {
        AnalogCalculusTrueNumber {
            content: vec![digit],
        }
    }

    pub fn truth_into_usize(self) -> usize {
        let mut r: usize = 0;
        for i in 0..self.content.len() {
            r = r.saturating_add(self.content[i] as usize * (10 * i));
        }
        r
    }

    pub fn true_add(self, other_truth: AnalogCalculusTrueNumber) -> Self {
        let mut new_self = self;
        for i in 0..new_self.content.len() {
            if other_truth.content.len() > new_self.content

            new_self.content[i] += other_truth.content[i];
            if new_self.content[i] > 9 {
                new_self.content[i] = 0;
                // The Analog truth is the truth even if it's the bottleneck!
                let mut carry_accumulator: usize = 1;
                loop {
                    carry_accumulator += 1;

                    if new_self.content.len() > (i + carry_accumulator) {
                        if new_self.content[i + carry_accumulator] < 8 {
                            new_self.content[i + carry_accumulator] += 1;
                            break;
                        } else {
                            new_self.content[i + carry_accumulator] = 0;
                        }
                    } else {
                        new_self.content.push(1);
                        break;
                    }
                }
            }
        }
        new_self
    }
}

#[repr(C, align(128))] // For crypto math, as true mathematicians need math centric code!
pub struct VirtualRunner {
    pub heap_cells: Vec<AnalogCalculusTrueNumber>,
    pub program_text: Vec<(
        ANALOGOPCODES,
        AnalogCalculusTrueNumber,
        AnalogCalculusTrueNumber,
    )>,
    pub not_your_stack: Vec<usize>,
}

fn main() {
    println!("Loading The-dot-language Interpreter...");
    let mut virtual_runner: VirtualRunner = VirtualRunner {
        heap_cells: Vec::new(),
        program_text: Vec::new(),
        not_your_stack: Vec::new(),
    };

    let mut looping_infinity_pc: usize = 0;
    loop {
        looping_infinity_pc = looping_infinity_pc.wrapping_add(1);

        if looping_infinity_pc > virtual_runner.program_text.len() {
            println!("Computer Daemon: Me and your hardware are done with your slopware!");
            println!("MEET THE PURE DISCRETE ENTROPY SINGULARITY!");
            // TODO infinite randomness

            let mut test_result = AnalogCalculusTrueNumber::create_digit_of_truth(4);
            loop {
                test_result =
                    test_result.true_add(AnalogCalculusTrueNumber::create_digit_of_truth(7));
                println!("{:?}", test_result);
            }
        }

        let arg0 = virtual_runner.program_text[looping_infinity_pc].1.clone();
        let arg1 = virtual_runner.program_text[looping_infinity_pc].2.clone();
        let arg0_usize = virtual_runner.program_text[looping_infinity_pc]
            .1
            .clone()
            .truth_into_usize()
            .clone();
        let arg1_usize = virtual_runner.program_text[looping_infinity_pc]
            .2
            .clone()
            .truth_into_usize()
            .clone();

        match virtual_runner.program_text[looping_infinity_pc].0 {
            ANALOGOPCODES::MOV => {
                virtual_runner.heap_cells[arg1_usize] =
                    virtual_runner.heap_cells[arg0_usize].clone();
            }
            ANALOGOPCODES::IMM => {
                virtual_runner.heap_cells[arg0_usize] = arg1;
            }
            ANALOGOPCODES::ADD => {
                virtual_runner.heap_cells[arg0_usize] = virtual_runner.heap_cells[arg0_usize]
                    .clone()
                    .true_add(virtual_runner.heap_cells[arg1_usize].clone());
            }
            ANALOGOPCODES::SUB => {}
            ANALOGOPCODES::MUL => {}
            ANALOGOPCODES::DIV => {}
            ANALOGOPCODES::SQRT => {}
            ANALOGOPCODES::RAISE => {}
            ANALOGOPCODES::CRTSUBROUTINES => {}
            ANALOGOPCODES::CALLSUBROUTINES => {}
            ANALOGOPCODES::JMP => {}
            ANALOGOPCODES::JCC => {}
            ANALOGOPCODES::JGT => {}
            ANALOGOPCODES::JLT => {}
            ANALOGOPCODES::NOP => {}
        }
    }
}
