use std::env;

const REG_NAMES: [[&str; 2]; 8] = [
    ["al", "ax"],
    ["cl", "cx"],
    ["dl", "dx"],
    ["bl", "bx"],
    ["ah", "sp"],
    ["ch", "bp"],
    ["dh", "si"],
    ["bh", "di"],
];

#[derive(Debug)]
enum Instruction {
    MOV,
}

fn decode_instruction(byte: u8) -> Instruction {
    match byte {
        0b00100010 => Instruction::MOV,
        _ => panic!("Unknown instruction"),
    }
}

#[derive(Debug)]
enum Direction {
    ToRegister,
    FromRegister,
}

fn decode_d(byte: u8) -> Direction {
    match byte {
        0b1 => Direction::ToRegister,
        0b0 => Direction::FromRegister,
        _ => panic!("invalid byte"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let loaded_bytes = std::fs::read(&args[1]).unwrap();
    for bytes in loaded_bytes.chunks(2) {
        // println!("first byte: {:#010b}", loaded_bytes[0]);
        // println!("second byte: {:#010b}", loaded_bytes[1]);

        let instruction_byte = bytes[0] >> 2;
        let d_byte = (bytes[0] >> 1) & 0b1;
        let w_byte = bytes[0] & 0b1;
        let mod_byte = bytes[1] >> 6;
        let reg_byte = (bytes[1] >> 3) & 0b111;
        let rm_byte = bytes[1] & 0b111;

        match decode_d(d_byte) {
            Direction::ToRegister => {
                println!(
                    "{:?} {} {}",
                    decode_instruction(instruction_byte),
                    REG_NAMES[reg_byte as usize][w_byte as usize],
                    REG_NAMES[rm_byte as usize][w_byte as usize]
                )
            }
            Direction::FromRegister => {
                println!(
                    "{:?} {} {}",
                    decode_instruction(instruction_byte),
                    REG_NAMES[rm_byte as usize][w_byte as usize],
                    REG_NAMES[reg_byte as usize][w_byte as usize]
                )
            }
        }
    }
}
