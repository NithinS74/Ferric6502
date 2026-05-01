use wasm_bindgen::prelude::*;

const CARRY: u8 = 0b0000_0001;
const ZERO: u8 = 0b0000_0010;
const INTERRUPT: u8 = 0b0000_0100;
const DECIMAL: u8 = 0b0000_1000;
const OVERFLOW: u8 = 0b0100_0000;
const NEGETIVE: u8 = 0b1000_0000;

pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

#[wasm_bindgen]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status_register: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    memory: [u8; 0x10000],
}

impl CPU {




    pub fn load_program(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn run_program(&mut self, program: Vec<u8>) {
        self.load_program(program);
        self.reset();
        self.interpret();
    }

    fn step_internal(&mut self) -> bool {
        let opcode: u8 = self.mem_read(self.program_counter);
        println!("op: {:X}", opcode);
        self.program_counter = self.program_counter.wrapping_add(1);
        match opcode {
                //ADC immediate
                0x69 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.adc(address);
                }
                //ADC ZeroPage
                0x65 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.adc(address);
                }
                //ADC ZeroPageX
                0x75 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.adc(address);
                }
                //ADC Absolute
                0x6D => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.adc(address);
                }
                //ADC AbsoluteX
                0x7D => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.adc(address);
                }
                //ADC AbsoluteY
                0x79 => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteY);
                    self.adc(address);
                }
                //ADC IndirectX
                0x61 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectX);
                    self.adc(address);
                }
                //ADC IndirectY
                0x71 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectY);
                    self.adc(address);
                }
                //AND immediate
                0x29 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.and(address);
                }
                //AND ZeroPage
                0x25 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.and(address);
                }
                //AND ZeroPageX
                0x35 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.and(address);
                }
                //AND Absolute
                0x2D => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.and(address);
                }
                //AND AbsoluteX
                0x3D => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.and(address);
                }
                //AND AbsoluteY
                0x39 => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteY);
                    self.and(address);
                }
                //AND IndirectX
                0x21 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectX);
                    self.and(address);
                }
                //AND IndirectY
                0x31 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectY);
                    self.and(address);
                }
                //ASL accumulator
                0x0A => {
                    self.set_carry_flag_bit7(self.register_a);
                    self.register_a <<= 1;
                    self.set_zero_flag(self.register_a);
                    self.set_negetive_flag(self.register_a);
                }
                //ASL ZeroPage
                0x06 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.asl(address);
                }
                //ASL ZeroPageX
                0x16 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.asl(address);
                }
                //ASL Absolute
                0x0E => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.asl(address);
                }
                //ASL AbsoluteX
                0x1E => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.asl(address);
                }
                //BCC relative
                0x90 => {
                    //& has higher precidience
                    self.branch_if_true(self.status_register & CARRY == 0);
                }
                //BCS relative
                0xB0 => {
                    self.branch_if_true(self.status_register & CARRY == CARRY);
                }
                //BEQ relative
                0xF0 => {
                    self.branch_if_true(self.status_register & ZERO == ZERO);
                }
                //BMI relative
                0x30 => {
                    self.branch_if_true(self.status_register & NEGETIVE == NEGETIVE);
                }
                //BNE relative
                0xD0 => {
                    self.branch_if_true(!(self.status_register & ZERO == ZERO));
                }
                //BPL relative
                0x10 => {
                    self.branch_if_true(!(self.status_register & NEGETIVE == NEGETIVE));
                }
                //BIT bit test ZeroPage
                0x24 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.bit(address);
                }
                //BIT bit test Absolute
                0x2C => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.bit(address);
                }
                //BVC relative
                0x50 => {
                    self.branch_if_true(self.status_register & OVERFLOW == 0);
                }
                //BVS relative
                0x70 => {
                    self.branch_if_true(self.status_register & OVERFLOW == OVERFLOW);
                }
                //CLC implied - carry clear
                0x18 => {
                    self.set_carry_flag_bit7(!NEGETIVE);
                }
                //CLI implied - INTERRUPT clear
                0x58 => {
                    self.status_register &= !INTERRUPT;
                }
                //CLV implied - OVERFLOW clear
                0xB8 => {
                    self.status_register &= !OVERFLOW;
                }
                //CMP immediate
                0xC9 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.cmp(address);
                }
                //CMP ZeroPage
                0xC5 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.cmp(address);
                }
                //CMP ZeroPageX
                0xD5 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.cmp(address);
                }
                //CMP Absolute
                0xCD => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.cmp(address);
                }
                //CMP AbsoluteX
                0xDD => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.cmp(address);
                }
                //CMP AbsoluteY
                0xD9 => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteY);
                    self.cmp(address);
                }
                //CMP IndirectX
                0xC1 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectX);
                    self.cmp(address);
                }
                //CMP IndirectY
                0xD1 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectY);
                    self.cmp(address);
                }
                //CPX immediate
                0xE0 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.cpx(address);
                }
                //CPX ZeroPage
                0xE4 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.cpx(address);
                }
                //CPX Absolute
                0xEC => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.cpx(address);
                }
                //CPY immediate
                0xC0 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.cpy(address);
                }
                //CPY ZeroPage
                0xC4 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.cpy(address);
                }
                //CPY Absolute
                0xCC => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.cpy(address);
                }
                //DEC ZeroPage
                0xC6 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.dec(address);
                }
                //DEC ZeroPageX
                0xD6 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.dec(address);
                }
                //DEC Absolute
                0xCE => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.dec(address);
                }
                //DEC AbsoluteX
                0xDE => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.dec(address);
                }
                //DEX implied
                0xCA => {
                    self.register_x = self.register_x.wrapping_sub(1);
                    self.set_negetive_flag(self.register_x);
                    self.set_zero_flag(self.register_x);
                }
                //DEY implied
                0x88 => {
                    self.register_y = self.register_y.wrapping_sub(1);
                    self.set_negetive_flag(self.register_y);
                    self.set_zero_flag(self.register_y);
                }

                //EOR Immediate - Exclusive OR
                0x49 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.eor(address);
                }
                //EOR ZeroPage - Exclusive OR
                0x45 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.eor(address);
                }
                //EOR ZeroPageX - Exclusive OR
                0x55 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.eor(address);
                }
                //EOR Absolute - Exclusive OR
                0x4D => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.eor(address);
                }
                //EOR AbsoluteX - Exclusive OR
                0x5D => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.eor(address);
                }
                //EOR AbsoluteY - Exclusive OR
                0x59 => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteY);
                    self.eor(address);
                }
                //EOR IndirectX - Exclusive OR
                0x41 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectX);
                    self.eor(address);
                }
                //EOR IndirectY - Exclusive OR
                0x51 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectY);
                    self.eor(address);
                }

                //INC ZeroPage
                0xE6 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.inc(address);
                }
                //INC ZeroPageX
                0xF6 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.inc(address);
                }
                //INC Absolute
                0xEE => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.inc(address);
                }
                //INC AbsoluteX
                0xFE => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.inc(address);
                }
                //INX implied opcode
                0xE8 => {
                    self.register_x = self.register_x.wrapping_add(1);
                    self.set_zero_flag(self.register_x);
                    self.set_negetive_flag(self.register_x);
                }
                //INY implied opcode
                0xC8 => {
                    self.register_y = self.register_y.wrapping_add(1);
                    self.set_zero_flag(self.register_y);
                    self.set_negetive_flag(self.register_y);
                }

                //JMP Absolute
                0x4C => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.program_counter = self.mem_read_u16(address);
                }
                //JMP Indirect
                0x6C => {
                    let address = self.get_operand_address(&AddressingMode::Indirect);
                    self.program_counter = self.mem_read_u16(address);
                }
                //JSR - Jump to subroutine
                0x20 => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.stack_push(self.program_counter);
                    self.program_counter = address;
                }
                //RTS - Return from subroutine
                0x60 => {
                    self.program_counter = self.stack_pop();
                }
                //Lda immediate opcode
                0xA9 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.lda(address);
                }
                //Lda ZeroPage opcode
                0xA5 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.lda(address);
                }
                //Lda ZeroPageX opcode
                0xB5 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.lda(address);
                }
                //Lda Absolute opcode
                0xAD => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.lda(address);
                }
                //Lda AbsoluteX opcode
                0xBD => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.lda(address);
                }
                //Lda AbsoluteY opcode
                0xB9 => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteY);
                    self.lda(address);
                }
                //Lda IndirectX opcode
                0xA1 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectX);
                    self.lda(address);
                }
                //Lda IndirectY opcode
                0xB1 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectY);
                    self.lda(address);
                }
                //ldx immediate opcode
                0xA2 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.ldx(address);
                }
                //ldx ZeroPage opcode
                0xA6 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.ldx(address);
                }
                //ldx ZeroPageY opcode
                0xB6 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageY);
                    self.ldx(address);
                }
                //ldx Absolute opcode
                0xAE => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.ldx(address);
                }
                //ldx AbsoluteY opcode
                0xBE => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteY);
                    self.ldx(address);
                }
                //ldy immediate opcode
                0xA0 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.ldy(address);
                }
                //ldy ZeroPage opcode
                0xA4 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.ldy(address);
                }
                //ldy ZeroPageX opcode
                0xB4 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.ldy(address);
                }
                //ldy Absolute opcode
                0xAC => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.ldy(address);
                }
                //ldy AbsoluteX opcode
                0xBC => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.ldy(address);
                }
                //LSR Accumulator
                0x4A => {
                    self.set_carry_flag_bit0(self.register_a);
                    self.register_a >>= 1;
                    self.set_zero_flag(self.register_a);
                    self.set_negetive_flag(self.register_a);
                }
                //LSR ZeroPage
                0x46 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.lsr(address);
                }
                //LSR ZeroPageX
                0x56 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.lsr(address);
                }
                //LSR Absolute
                0x4E => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.lsr(address);
                }
                //LSR AbsoluteX
                0x5E => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.lsr(address);
                }
                //NOP implied
                0xEA => {
                    {}
                }
                //ORA Immediate - Logical inclusive OR
                0x09 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.ora(address);
                }
                //ORA ZeroPage - Logical inclusive OR
                0x05 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.ora(address);
                }
                //ORA ZeroPageX - Logical inclusive OR
                0x15 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.ora(address);
                }
                //ORA Absolute - Logical inclusive OR
                0x0D => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.ora(address);
                }
                //ORA AbsoluteX - Logical inclusive OR
                0x1D => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.ora(address);
                }
                //ORA AbsoluteY - Logical inclusive OR
                0x19 => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteY);
                    self.ora(address);
                }
                //ORA IndirectX - Logical inclusive OR
                0x01 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectX);
                    self.ora(address);
                }
                //ORA IndirectY - Logical inclusive OR
                0x11 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectY);
                    self.ora(address);
                }
                //PHA implied
                0x48 => {
                    self.stack_push_u8(self.register_a);
                }
                //PHP implied
                0x08 => {
                    self.stack_push_u8(self.status_register);
                }
                //PLA implied
                0x68 => {
                    self.register_a = self.stack_pop_u8();
                    self.set_zero_flag(self.register_a);
                    self.set_negetive_flag(self.register_a);
                }
                //PLP implied
                0x28 => {
                    self.status_register = self.stack_pop_u8();
                }

                //ROL Accumulator
                0x2A => {
                    let temp = self.status_register;
                    self.set_carry_flag_bit7(self.register_a);
                    self.register_a <<= 1;
                    self.register_a |= temp & CARRY;
                    self.set_zero_flag(self.register_a);
                    self.set_negetive_flag(self.register_a);
                }

                //ROL ZeroPage
                0x26 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.rol(address);
                }
                //ROL ZeroPageX
                0x36 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.rol(address);
                }
                //ROL Absolute
                0x2E => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.rol(address);
                }
                //ROL AbsoluteX
                0x3E => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.rol(address);
                }

                //ROR Accumulator
                0x6A => {
                    let temp = self.status_register;
                    self.set_carry_flag_bit0(self.register_a);
                    self.register_a >>= 1;
                    self.register_a |= (temp << 7) & NEGETIVE;
                    self.set_zero_flag(self.register_a);
                    self.set_negetive_flag(self.register_a);
                }
                //ROR ZeroPage
                0x66 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.ror(address);
                }
                //ROR ZeroPageX
                0x76 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.ror(address);
                }
                //ROR Absolute
                0x6E => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.ror(address);
                }
                //ROR AbsoluteX
                0x7E => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.ror(address);
                }
                //RTI implied
                0x40 => {
                    self.status_register = self.stack_pop_u8();
                    self.program_counter = self.stack_pop();
                }

                //SBC immediate
                0xE9 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.sbc(address);
                }
                //SBC ZeroPage
                0xE5 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.sbc(address);
                }
                //SBC ZeroPageX
                0xF5 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.sbc(address);
                }
                //SBC Absolute
                0xED => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.sbc(address);
                }
                //SBC AbsoluteX
                0xFD => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.sbc(address);
                }
                //SBC AbsoluteY
                0xF9 => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteY);
                    self.sbc(address);
                }
                //SBC IndirectX
                0xE1 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectX);
                    self.sbc(address);
                }
                //SBC IndirectY
                0xF1 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectY);
                    self.sbc(address);
                }

                //SEC implied
                0x38 => {
                    self.set_carry_flag_bit0(CARRY);
                }
                //SED implied
                0xF8 => {
                    self.status_register |= DECIMAL;
                }
                //SEI implied
                0x78 => {
                    self.status_register |= INTERRUPT;
                }
                //STA ZeroPage
                0x85 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.sta(address);
                }
                //STA ZeroPageX
                0x95 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.sta(address);
                }
                //STA Absolute
                0x8D => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.sta(address);
                }
                //STA AbsoluteX
                0x9D => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.sta(address);
                }
                //STA AbsoluteY
                0x99 => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteY);
                    self.sta(address);
                }
                //STA IndirectX
                0x81 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectX);
                    self.sta(address);
                }
                //STA IndirectY
                0x91 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectY);
                    self.sta(address);
                }
                //STX ZeroPage
                0x86 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.stx(address);
                }
                //STX ZeroPageY
                0x96 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageY);
                    self.stx(address);
                }
                //STX Absolute
                0x8E => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.stx(address);
                }
                //STY ZeroPage
                0x84 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.sty(address);
                }
                //STY ZeroPageY
                0x94 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageY);
                    self.sty(address);
                }
                //STY Absolute
                0x8C => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.sty(address);
                }
                //TAX implied opcode
                0xAA => {
                    self.register_x = self.register_a;
                    self.set_zero_flag(self.register_a);
                    self.set_negetive_flag(self.register_a);
                }
                //TAY implied opcode
                0xA8 => {
                    self.register_y = self.register_a;
                    self.set_zero_flag(self.register_a);
                    self.set_negetive_flag(self.register_a);
                }
                //TSX implied
                0xBA => {
                    self.register_x = self.stack_pointer;
                    self.set_zero_flag(self.register_x);
                    self.set_negetive_flag(self.register_x);
                }
                //TXA implied
                0x8A => {
                    self.register_a = self.register_x;
                    self.set_zero_flag(self.register_a);
                    self.set_negetive_flag(self.register_a);
                }
                //TXS implied
                0x9A => {
                    self.stack_pointer = self.register_x;
                }
                //TYA implied
                0x98 => {
                    self.register_a = self.register_y;
                    self.set_zero_flag(self.register_a);
                    self.set_negetive_flag(self.register_a);
                }

                //BRK opcode
                0x00 => {
                    return true;
                }
                _ => {
                    println!("Entered wild branch with opcode {:X}", opcode);
                    panic!("wild branch");
                }
            }
        false
    }

    pub fn interpret(&mut self) {
        while !self.step_internal() {}
    }

    fn branch_if_true(&mut self, value: bool) {
        if value {
            let address = self.get_operand_address(&AddressingMode::Immediate);
            let value = self.mem_read(address);
            //-1 to reduce the +1 that we updated from getting the address
            self.program_counter = self.program_counter.wrapping_add((value - 1) as u16);
        } else {
            self.program_counter = self.program_counter.wrapping_add(1);
        }
    }

    fn asl(&mut self, address: u16) {
        let mut mem_value = self.mem_read(address);
        self.set_carry_flag_bit7(mem_value);
        mem_value <<= 1;
        self.mem_write(address, mem_value);
        self.set_zero_flag(mem_value);
        self.set_negetive_flag(mem_value);
    }

    fn and(&mut self, address: u16) {
        let value = self.mem_read(address);
        self.register_a &= value;
        self.set_zero_flag(self.register_a);
        self.set_negetive_flag(self.register_a);
    }

    fn ora(&mut self, address: u16) {
        let value = self.mem_read(address);
        self.register_a |= value;
        self.set_zero_flag(self.register_a);
        self.set_negetive_flag(self.register_a);
    }
    fn eor(&mut self, address: u16) {
        let value = self.mem_read(address);
        self.register_a ^= value;
        self.set_zero_flag(self.register_a);
        self.set_negetive_flag(self.register_a);
    }

    fn cmp(&mut self, address: u16) {
        let m = self.mem_read(address);
        let value = self.register_a.wrapping_sub(m);
        self.set_zero_flag(value);
        self.set_negetive_flag(value);
        if self.register_a >= m {
            self.status_register |= CARRY;
        } else {
            self.status_register &= !CARRY;
        }
    }

    fn cpx(&mut self, address: u16) {
        let m = self.mem_read(address);
        let value = self.register_x.wrapping_sub(m);
        self.set_zero_flag(value);
        self.set_negetive_flag(value);
        if self.register_x >= m {
            self.status_register |= CARRY;
        } else {
            self.status_register &= !CARRY;
        }
    }

    fn cpy(&mut self, address: u16) {
        let m = self.mem_read(address);
        let value = self.register_y.wrapping_sub(m);
        self.set_zero_flag(value);
        self.set_negetive_flag(value);
        if self.register_y >= m {
            self.status_register |= CARRY;
        } else {
            self.status_register &= !CARRY;
        }
    }

    fn inc(&mut self, address: u16) {
        let m: u8 = self.mem_read(address);
        let value = m.wrapping_add(1);
        self.mem_write(address, value);
        self.set_zero_flag(value);
        self.set_negetive_flag(value);
    }
    fn dec(&mut self, address: u16) {
        let m: u8 = self.mem_read(address);
        let value = m.wrapping_sub(1);
        self.mem_write(address, value);
        self.set_zero_flag(value);
        self.set_negetive_flag(value);
    }

    fn stack_pop_u8(&mut self) -> u8 {
        if self.stack_pointer == 0xff {
            panic!("stack underflow");
        }
        let address = self.mem_read(0x0100 + self.stack_pointer as u16);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        address
    }
    fn stack_push_u8(&mut self, addr: u8) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.mem_write(0x0100 + self.stack_pointer as u16, addr);
    }

    fn stack_pop(&mut self) -> u16 {
        if self.stack_pointer == 0xff {
            panic!("stack underflow");
        }
        let address = self.mem_read_u16(0x0100 + self.stack_pointer as u16);
        self.stack_pointer = self.stack_pointer.wrapping_add(2);
        address
    }

    fn stack_push(&mut self, addr: u16) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(2);
        self.mem_write_u16(0x0100 + self.stack_pointer as u16, addr);
    }

    fn lda(&mut self, address: u16) {
        let value = self.mem_read(address);
        self.register_a = value;
        self.set_zero_flag(value);
        self.set_negetive_flag(value);
    }

    fn ldx(&mut self, address: u16) {
        let value = self.mem_read(address);
        self.register_x = value;
        self.set_zero_flag(value);
        self.set_negetive_flag(value);
    }

    fn ldy(&mut self, address: u16) {
        let value = self.mem_read(address);
        self.register_y = value;
        self.set_zero_flag(value);
        self.set_negetive_flag(value);
    }

    fn lsr(&mut self, address: u16) {
        let mut mem_value = self.mem_read(address);
        self.set_carry_flag_bit0(mem_value);
        mem_value >>= 1;
        self.mem_write(address, mem_value);
        self.set_zero_flag(mem_value);
        self.set_negetive_flag(mem_value);
    }
    fn bit(&mut self, address: u16) {
        let value = self.mem_read(address);
        let result = self.register_a & value;
        self.set_zero_flag(result);
        self.set_negetive_flag(value);
        self.set_overflow_flag(value);
    }

    fn rol(&mut self, address: u16) {
        let mut mem_value = self.mem_read(address);
        let temp = self.status_register;
        self.set_carry_flag_bit7(mem_value);
        mem_value <<= 1;
        mem_value |= temp & CARRY;
        self.set_zero_flag(mem_value);
        self.set_negetive_flag(mem_value);
        self.mem_write(address, mem_value);
    }
    fn ror(&mut self, address: u16) {
        let mut mem_value = self.mem_read(address);
        let temp = self.status_register;
        self.set_carry_flag_bit0(mem_value);
        mem_value >>= 1;
        mem_value |= (temp & CARRY) << 7;
        self.set_zero_flag(mem_value);
        self.set_negetive_flag(mem_value);
        self.mem_write(address, mem_value);
    }

    fn adc(&mut self, address: u16) {
        let value = self.mem_read(address);

        let carry = (self.status_register & CARRY) as u16;
        let sum = self.register_a as u16 + value as u16 + carry;
        let result = sum as u8;

        // Carry flag
        if sum > 0xFF {
            self.status_register |= CARRY;
        } else {
            self.status_register &= !CARRY;
        }

        // Overflow flag
        let a = self.register_a;
        let m = value;
        let r = result;

        if ((a ^ r) & (m ^ r) & 0x80) != 0 {
            self.status_register |= OVERFLOW;
        } else {
            self.status_register &= !OVERFLOW;
        }

        self.register_a = result;

        self.set_zero_flag(self.register_a);
        self.set_negetive_flag(self.register_a);
    }

    fn sbc(&mut self, address: u16) {
        let value = self.mem_read(address);
        let inverted = value ^ 0xFF;
        let sum = self.register_a as u16 + inverted as u16 + (self.status_register & CARRY) as u16;

        let result = sum as u8;
        if sum > 0xFF {
            self.status_register |= CARRY;
        } else {
            self.status_register &= !CARRY;
        }
        let a = self.register_a;
        let m = value;
        let r = result;
        if ((a ^ r) & (a ^ m) & 0x80) != 0 {
            self.status_register |= OVERFLOW;
        } else {
            self.status_register &= !OVERFLOW;
        }
        self.register_a = result;
        self.set_zero_flag(self.register_a);
        self.set_negetive_flag(self.register_a);
    }

    fn sta(&mut self, address: u16) {
        self.mem_write(address, self.register_a);
    }
    fn stx(&mut self, address: u16) {
        self.mem_write(address, self.register_x);
    }
    fn sty(&mut self, address: u16) {
        self.mem_write(address, self.register_y);
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => {
                self.program_counter = self.program_counter.wrapping_add(1);
                self.program_counter.wrapping_sub(1)
            }

            AddressingMode::ZeroPage => {
                self.program_counter = self.program_counter.wrapping_add(1);
                self.mem_read(self.program_counter.wrapping_sub(1)) as u16
            }

            AddressingMode::ZeroPageX => {
                self.program_counter = self.program_counter.wrapping_add(1);
                self.mem_read(self.program_counter.wrapping_sub(1))
                    .wrapping_add(self.register_x) as u16
            }

            AddressingMode::ZeroPageY => {
                self.program_counter = self.program_counter.wrapping_add(1);
                self.mem_read(self.program_counter.wrapping_sub(1))
                    .wrapping_add(self.register_y) as u16
            }

            AddressingMode::Absolute => {
                self.program_counter = self.program_counter.wrapping_add(2);
                self.mem_read_u16(self.program_counter.wrapping_sub(2))
            }

            AddressingMode::AbsoluteX => {
                self.program_counter = self.program_counter.wrapping_add(2);
                self.mem_read_u16(self.program_counter.wrapping_sub(2))
                    .wrapping_add(self.register_x as u16)
            }

            AddressingMode::AbsoluteY => {
                self.program_counter = self.program_counter.wrapping_add(2);
                self.mem_read_u16(self.program_counter.wrapping_sub(2))
                    .wrapping_add(self.register_y as u16)
            }

            AddressingMode::Indirect => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let ptr = self.mem_read(self.program_counter.wrapping_sub(1));
                self.mem_read_u16(ptr as u16)
            }

            AddressingMode::IndirectX => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let ptr = self.mem_read(self.program_counter.wrapping_sub(1));
                self.mem_read_u16(ptr.wrapping_add(self.register_x) as u16)
            }

            AddressingMode::IndirectY => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let ptr = self.mem_read(self.program_counter.wrapping_sub(1));
                self.mem_read_u16(ptr as u16)
                    .wrapping_add(self.register_y as u16)
            }

            AddressingMode::NoneAddressing => {
                panic!("Address mode error: NoneAddressing provided");
            }
        }
    }

    fn mem_read_u16(&mut self, addr: u16) -> u16 {
        let addr = addr as usize;
        let bytes: [u8; 2] = [self.memory[addr], self.memory[addr + 1]];
        u16::from_le_bytes(bytes)
    }

    fn mem_write_u16(&mut self, addr: u16, value: u16) {
        let addr = addr as usize;
        let [a, b]: [u8; 2] = value.to_le_bytes();
        self.memory[addr] = a;
        self.memory[addr + 1] = b;
    }

    fn mem_read(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    fn mem_write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    fn set_overflow_flag(&mut self, value: u8) {
        if value & OVERFLOW == OVERFLOW {
            self.status_register |= OVERFLOW;
        } else {
            self.status_register &= !OVERFLOW;
        }
    }
    fn set_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.status_register |= ZERO;
        } else {
            self.status_register &= !ZERO;
        }
    }

    fn set_carry_flag_bit0(&mut self, value: u8) {
        if value & CARRY == CARRY {
            self.status_register |= CARRY;
        } else {
            self.status_register &= !CARRY;
        }
    }
    fn set_carry_flag_bit7(&mut self, value: u8) {
        if value & NEGETIVE == NEGETIVE {
            self.status_register |= CARRY;
        } else {
            self.status_register &= !CARRY;
        }
    }

    fn set_negetive_flag(&mut self, value: u8) {
        if value & NEGETIVE == NEGETIVE {
            self.status_register |= NEGETIVE;
        } else {
            self.status_register &= !NEGETIVE;
        }
        return;
    }
}


#[wasm_bindgen]
impl CPU {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status_register: 0,
            program_counter: 0,
            stack_pointer: 0xff,
            memory: [0 as u8; 0x10000],
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status_register = 0;
        self.stack_pointer = 0xff;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn step(&mut self) -> bool {
        self.step_internal()
    }

    pub fn get_register_a(&self) -> u8 {
        self.register_a
    }

    pub fn get_register_x(&self) -> u8 {
        self.register_x
    }

    pub fn get_register_y(&self) -> u8 {
        self.register_y
    }

    pub fn get_status_register(&self) -> u8 {
        self.status_register
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn get_stack_pointer(&self) -> u8 {
        self.stack_pointer
    }

    pub fn get_memory_slice(&self, start: u16, length: u16) -> Vec<u8> {
        let start = start as usize;
        let length = length as usize;
        self.memory[start..start + length].to_vec()
    }

    pub fn load_program_from_js(&mut self, program: &[u8]) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(program);
        self.mem_write_u16(0xFFFC, 0x8000);
    }
}

#[cfg(test)]
#[path = "./cpu_test.rs"]
mod cpu_tests;
