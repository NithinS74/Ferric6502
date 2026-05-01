use super::*;

// ============================================================
// HELPERS
// ============================================================

const ALL_FLAGS: u8 = CARRY | ZERO | INTERRUPT | DECIMAL | OVERFLOW | NEGETIVE;

fn assert_flag(cpu: &CPU, flag: u8, expected: bool, context: &str) {
    let actual = cpu.status_register & flag == flag;
    assert!(
        actual == expected,
        "Flag {:08b} ({:?}): expected {} but was {}. Context: {}. Full status: {:08b}",
        flag,
        match flag {
            CARRY => "CARRY",
            ZERO => "ZERO",
            INTERRUPT => "INTERRUPT",
            DECIMAL => "DECIMAL",
            OVERFLOW => "OVERFLOW",
            NEGETIVE => "NEGATIVE",
            _ => "UNKNOWN",
        },
        expected,
        actual,
        context,
        cpu.status_register
    );
}

fn assert_flags(cpu: &CPU, expected: u8, mask: u8, context: &str) {
    let actual_masked = cpu.status_register & mask;
    assert!(
        actual_masked == expected & mask,
        "Flags mismatch. Expected (masked): {:08b}, Actual: {:08b}, Mask: {:08b}. Context: {}",
        expected & mask,
        actual_masked,
        mask,
        context
    );
}

/// Helper: load program, reset CPU, optionally set up state, then interpret
fn run_program_with_setup<F>(program: Vec<u8>, setup: F) -> CPU
where
    F: FnOnce(&mut CPU),
{
    let mut cpu = CPU::new();
    cpu.load_program(program);
    cpu.reset();
    setup(&mut cpu);
    cpu.interpret();
    cpu
}

/// Helper: run program directly (no extra setup needed)
fn run_program(program: Vec<u8>) -> CPU {
    let mut cpu = CPU::new();
    cpu.run_program(program);
    cpu
}

/// Helper: run AND/OR/EOR/shift with A register value set
fn run_with_a(instruction: u8, operand: u8, a_val: u8) -> CPU {
    run_program_with_setup(vec![instruction, operand, 0x00], |cpu| {
        cpu.register_a = a_val;
    })
}

/// Helper: run instruction with X register value set
fn run_with_x(instruction: u8, operand: u8, x_val: u8) -> CPU {
    run_program_with_setup(vec![instruction, operand, 0x00], |cpu| {
        cpu.register_x = x_val;
    })
}

/// Helper: run instruction with Y register value set
fn run_with_y(instruction: u8, operand: u8, y_val: u8) -> CPU {
    run_program_with_setup(vec![instruction, operand, 0x00], |cpu| {
        cpu.register_y = y_val;
    })
}

// ============================================================
// LDA - Load Accumulator
// ============================================================

#[test]
fn test_lda_immediate_zero_flag() {
    let cpu = run_program(vec![0xa9, 0x00, 0x00]);
    assert_eq!(cpu.register_a, 0x00, "LDA immediate: A should be 0");
    assert_flag(&cpu, ZERO, true, "LDA #$00 should set ZERO");
    assert_flag(&cpu, NEGETIVE, false, "LDA #$00 should clear NEGATIVE");
}

#[test]
fn test_lda_immediate_negative_flag() {
    let cpu = run_program(vec![0xa9, 0x80, 0x00]);
    assert_eq!(cpu.register_a, 0x80, "LDA immediate: A should be 0x80");
    assert_flag(&cpu, NEGETIVE, true, "LDA #$80 should set NEGATIVE");
    assert_flag(&cpu, ZERO, false, "LDA #$80 should clear ZERO");
}

#[test]
fn test_lda_immediate_boundary_7f() {
    let cpu = run_program(vec![0xa9, 0x7f, 0x00]);
    assert_eq!(cpu.register_a, 0x7f);
    assert_flag(&cpu, NEGETIVE, false, "0x7F should not set NEGATIVE");
    assert_flag(&cpu, ZERO, false, "0x7F should not set ZERO");
}

#[test]
fn test_lda_immediate_boundary_ff() {
    let cpu = run_program(vec![0xa9, 0xff, 0x00]);
    assert_eq!(cpu.register_a, 0xff);
    assert_flag(&cpu, NEGETIVE, true, "0xFF should set NEGATIVE");
    assert_flag(&cpu, ZERO, false, "0xFF should not set ZERO");
}

#[test]
fn test_lda_immediate_boundary_01() {
    let cpu = run_program(vec![0xa9, 0x01, 0x00]);
    assert_eq!(cpu.register_a, 0x01);
    assert_flag(&cpu, NEGETIVE, false, "0x01 should not set NEGATIVE");
    assert_flag(&cpu, ZERO, false, "0x01 should not set ZERO");
}

#[test]
fn test_lda_zeropage_basic() {
    let cpu = run_program_with_setup(vec![0xa5, 0x42, 0x00], |cpu| {
        cpu.memory[0x42] = 0x37;
    });
    assert_eq!(cpu.register_a, 0x37);
    assert_flag(&cpu, ZERO, false, "LDA $42=0x37 should not set ZERO");
}

#[test]
fn test_lda_zeropage_zero() {
    let cpu = run_program_with_setup(vec![0xa5, 0x42, 0x00], |cpu| {
        cpu.memory[0x42] = 0x00;
    });
    assert_eq!(cpu.register_a, 0x00);
    assert_flag(&cpu, ZERO, true, "LDA $42=0x00 should set ZERO");
}

#[test]
fn test_lda_zeropage_negative() {
    let cpu = run_program_with_setup(vec![0xa5, 0x42, 0x00], |cpu| {
        cpu.memory[0x42] = 0x80;
    });
    assert_eq!(cpu.register_a, 0x80);
    assert_flag(&cpu, NEGETIVE, true, "LDA $42=0x80 should set NEGATIVE");
}

#[test]
fn test_lda_zeropagex_basic() {
    let cpu = run_program_with_setup(vec![0xb5, 0x42, 0x00], |cpu| {
        cpu.register_x = 0x05;
        cpu.memory[0x47] = 0x33;
    });
    assert_eq!(cpu.register_a, 0x33);
}

#[test]
fn test_lda_zeropagex_wrap() {
    let cpu = run_program_with_setup(vec![0xb5, 0x42, 0x00], |cpu| {
        cpu.register_x = 0xff;
        cpu.memory[0x41] = 0x99;
    });
    assert_eq!(cpu.register_a, 0x99, "ZeroPageX wrap: 0x42 + 0xFF = 0x41");
}

#[test]
fn test_lda_zeropagex_wrap_exact() {
    let cpu = run_program_with_setup(vec![0xb5, 0xff, 0x00], |cpu| {
        cpu.register_x = 0x01;
        cpu.memory[0x00] = 0xAA;
    });
    assert_eq!(cpu.register_a, 0xAA, "ZeroPageX wrap: 0xFF + 1 = 0x00");
}

#[test]
fn test_lda_absolute_basic() {
    let cpu = run_program_with_setup(vec![0xad, 0x34, 0x12, 0x00], |cpu| {
        cpu.memory[0x1234] = 0x42;
    });
    assert_eq!(cpu.register_a, 0x42);
}

#[test]
fn test_lda_absolutex_basic() {
    let cpu = run_program_with_setup(vec![0xbd, 0x34, 0x12, 0x00], |cpu| {
        cpu.register_x = 0x10;
        cpu.memory[0x1244] = 0x55;
    });
    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_lda_absolutex_wrap_ff() {
    let cpu = run_program_with_setup(vec![0xbd, 0xff, 0xff, 0x00], |cpu| {
        cpu.register_x = 0x01;
        cpu.memory[0x0000] = 0x77;
    });
    assert_eq!(
        cpu.register_a, 0x77,
        "AbsoluteX 16-bit wrap: 0xFFFE + 1 = 0x0000"
    );
}

#[test]
fn test_lda_absolutey_basic() {
    let cpu = run_program_with_setup(vec![0xb9, 0x34, 0x12, 0x00], |cpu| {
        cpu.register_y = 0x20;
        cpu.memory[0x1254] = 0x66;
    });
    assert_eq!(cpu.register_a, 0x66);
}

#[test]
fn test_lda_indirectx_basic() {
    let cpu = run_program_with_setup(vec![0xa1, 0x40, 0x00], |cpu| {
        cpu.register_x = 0x04;
        cpu.memory[0x44] = 0x34;
        cpu.memory[0x45] = 0x12;
        cpu.memory[0x1234] = 0x99;
    });
    assert_eq!(cpu.register_a, 0x99);
}

#[test]
fn test_lda_indirectx_zeropage_wrap() {
    let cpu = run_program_with_setup(vec![0xa1, 0xff, 0x00], |cpu| {
        cpu.register_x = 0x01;
        cpu.memory[0x00] = 0x34;
        cpu.memory[0x01] = 0x12;
        cpu.memory[0x1234] = 0xAB;
    });
    assert_eq!(cpu.register_a, 0xAB, "IndirectX pointer wrap 0xFF+1=0x00");
}

#[test]
fn test_lda_indirecty_basic() {
    // LDA ($30),Y
    // Pointer at 0x30: low=0x40, high=0x12 => base address = 0x1240
    // Y = 0x10 => final address = 0x1240 + 0x10 = 0x1250
    let cpu = run_program_with_setup(vec![0xb1, 0x30, 0x00], |cpu| {
        cpu.register_y = 0x10;
        cpu.memory[0x30] = 0x40; // Low byte of pointer at 0x30
        cpu.memory[0x31] = 0x12; // High byte of pointer at 0x31
        cpu.memory[0x1250] = 0x77; // 0x1240 + 0x10
    });
    assert_eq!(cpu.register_a, 0x77);
}

#[test]
fn test_lda_does_not_affect_x_y() {
    let cpu = run_program_with_setup(vec![0xa9, 0x42, 0x00], |cpu| {
        cpu.register_x = 0x33;
        cpu.register_y = 0x44;
    });
    assert_eq!(cpu.register_x, 0x33, "LDA should not modify X");
    assert_eq!(cpu.register_y, 0x44, "LDA should not modify Y");
}

// ============================================================
// LDX - Load X Register
// ============================================================

#[test]
fn test_ldx_immediate_zero() {
    let cpu = run_program(vec![0xa2, 0x00, 0x00]);
    assert_eq!(cpu.register_x, 0x00);
    assert_flag(&cpu, ZERO, true, "LDX #$00 should set ZERO");
    assert_flag(&cpu, NEGETIVE, false, "LDX #$00 should clear NEGATIVE");
}

#[test]
fn test_ldx_immediate_negative() {
    let cpu = run_program(vec![0xa2, 0x80, 0x00]);
    assert_eq!(cpu.register_x, 0x80);
    assert_flag(&cpu, NEGETIVE, true, "LDX #$80 should set NEGATIVE");
}

#[test]
fn test_ldx_zeropage_basic() {
    let cpu = run_program_with_setup(vec![0xa6, 0x30, 0x00], |cpu| {
        cpu.memory[0x30] = 0x22;
    });
    assert_eq!(cpu.register_x, 0x22);
}

#[test]
fn test_ldx_zeropagey_basic() {
    let cpu = run_program_with_setup(vec![0xb6, 0x30, 0x00], |cpu| {
        cpu.register_y = 0x03;
        cpu.memory[0x33] = 0x44;
    });
    assert_eq!(cpu.register_x, 0x44);
}

#[test]
fn test_ldx_zeropagey_wrap() {
    let cpu = run_program_with_setup(vec![0xb6, 0x30, 0x00], |cpu| {
        cpu.register_y = 0xff;
        cpu.memory[0x2f] = 0x55;
    });
    assert_eq!(
        cpu.register_x, 0x55,
        "LDX ZeroPageY wrap: 0x30 + 0xFF = 0x2F"
    );
}

#[test]
fn test_ldx_absolute_basic() {
    let cpu = run_program_with_setup(vec![0xae, 0x00, 0x26, 0x00], |cpu| {
        cpu.memory[0x2600] = 0x33;
    });
    assert_eq!(cpu.register_x, 0x33);
}

#[test]
fn test_ldx_absolutey_basic() {
    let cpu = run_program_with_setup(vec![0xbe, 0x00, 0x26, 0x00], |cpu| {
        cpu.register_y = 0x10;
        cpu.memory[0x2610] = 0x44;
    });
    assert_eq!(cpu.register_x, 0x44);
}

#[test]
fn test_ldx_does_not_affect_a_y() {
    let cpu = run_program_with_setup(vec![0xa2, 0x42, 0x00], |cpu| {
        cpu.register_a = 0x55;
        cpu.register_y = 0x66;
    });
    assert_eq!(cpu.register_a, 0x55, "LDX should not modify A");
    assert_eq!(cpu.register_y, 0x66, "LDX should not modify Y");
}

// ============================================================
// LDY - Load Y Register
// ============================================================

#[test]
fn test_ldy_immediate_zero() {
    let cpu = run_program(vec![0xa0, 0x00, 0x00]);
    assert_eq!(cpu.register_y, 0x00);
    assert_flag(&cpu, ZERO, true, "LDY #$00 should set ZERO");
}

#[test]
fn test_ldy_immediate_negative() {
    let cpu = run_program(vec![0xa0, 0x80, 0x00]);
    assert_eq!(cpu.register_y, 0x80);
    assert_flag(&cpu, NEGETIVE, true, "LDY #$80 should set NEGATIVE");
}

#[test]
fn test_ldy_zeropage_basic() {
    let cpu = run_program_with_setup(vec![0xa4, 0x50, 0x00], |cpu| {
        cpu.memory[0x50] = 0x11;
    });
    assert_eq!(cpu.register_y, 0x11);
}

#[test]
fn test_ldy_zeropagex_basic() {
    let cpu = run_program_with_setup(vec![0xb4, 0x50, 0x00], |cpu| {
        cpu.register_x = 0x05;
        cpu.memory[0x55] = 0x22;
    });
    assert_eq!(cpu.register_y, 0x22);
}

#[test]
fn test_ldy_absolute_basic() {
    let cpu = run_program_with_setup(vec![0xac, 0x00, 0x34, 0x00], |cpu| {
        cpu.memory[0x3400] = 0x66;
    });
    assert_eq!(cpu.register_y, 0x66);
}

#[test]
fn test_ldy_absolutex_basic() {
    let cpu = run_program_with_setup(vec![0xbc, 0x00, 0x34, 0x00], |cpu| {
        cpu.register_x = 0x20;
        cpu.memory[0x3420] = 0x77;
    });
    assert_eq!(cpu.register_y, 0x77);
}

#[test]
fn test_ldy_does_not_affect_a_x() {
    let cpu = run_program_with_setup(vec![0xa0, 0x42, 0x00], |cpu| {
        cpu.register_a = 0x55;
        cpu.register_x = 0x66;
    });
    assert_eq!(cpu.register_a, 0x55, "LDY should not modify A");
    assert_eq!(cpu.register_x, 0x66, "LDY should not modify X");
}

// ============================================================
// AND - Logical AND
// ============================================================

#[test]
fn test_and_immediate_basic() {
    let cpu = run_with_a(0x29, 0x0F, 0xA5);
    assert_eq!(cpu.register_a, 0x05, "AND #$0F with A=0xA5 = 0x05");
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "0x05 is not negative");
}

#[test]
fn test_and_immediate_zero_result() {
    let cpu = run_with_a(0x29, 0x0F, 0x80);
    assert_eq!(cpu.register_a, 0x00, "AND #$0F with A=0x80 = 0x00");
    assert_flag(&cpu, ZERO, true, "Zero result");
    assert_flag(&cpu, NEGETIVE, false, "Zero clears NEGATIVE");
}

#[test]
fn test_and_immediate_negative_result() {
    let cpu = run_with_a(0x29, 0xF0, 0x88);
    assert_eq!(cpu.register_a, 0x80, "AND #$F0 with A=0x88 = 0x80");
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
}

#[test]
fn test_and_immediate_all_ones() {
    let cpu = run_with_a(0x29, 0xFF, 0x55);
    assert_eq!(cpu.register_a, 0x55, "AND #$FF is identity");
}

#[test]
fn test_and_immediate_all_zeros() {
    let cpu = run_with_a(0x29, 0x00, 0xFF);
    assert_eq!(cpu.register_a, 0x00, "AND #$00 gives 0");
    assert_flag(&cpu, ZERO, true, "Zero result");
}

#[test]
fn test_and_zeropage() {
    let cpu = run_program_with_setup(vec![0x25, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xF0;
        cpu.memory[0x44] = 0x0F;
    });
    assert_eq!(cpu.register_a, 0x00, "AND $44: 0xF0 & 0x0F = 0x00");
    assert_flag(&cpu, ZERO, true, "Zero result");
}

#[test]
fn test_and_zeropagex() {
    let cpu = run_program_with_setup(vec![0x35, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xF0;
        cpu.register_x = 0x04;
        cpu.memory[0x48] = 0x0F;
    });
    assert_eq!(cpu.register_a, 0x00, "AND $44,X: 0xF0 & 0x0F = 0x00");
}

#[test]
fn test_and_absolute() {
    let cpu = run_program_with_setup(vec![0x2D, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.memory[0x4000] = 0x0F;
    });
    assert_eq!(cpu.register_a, 0x0A, "AND $4000: 0xAA & 0x0F = 0x0A");
}

#[test]
fn test_and_absolutex() {
    let cpu = run_program_with_setup(vec![0x3D, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_x = 0x10;
        cpu.memory[0x4010] = 0x0F;
    });
    assert_eq!(cpu.register_a, 0x0A, "AND $4000,X: 0xAA & 0x0F = 0x0A");
}

#[test]
fn test_and_absolutey() {
    let cpu = run_program_with_setup(vec![0x39, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_y = 0x10;
        cpu.memory[0x4010] = 0x0F;
    });
    assert_eq!(cpu.register_a, 0x0A, "AND $4000,Y: 0xAA & 0x0F = 0x0A");
}

#[test]
fn test_and_indirectx() {
    let cpu = run_program_with_setup(vec![0x21, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_x = 0x02;
        cpu.memory[0x46] = 0x00;
        cpu.memory[0x47] = 0x40;
        cpu.memory[0x4000] = 0x0F;
    });
    assert_eq!(cpu.register_a, 0x0A, "AND ($44,X): 0xAA & 0x0F = 0x0A");
}

#[test]
fn test_and_indirecty() {
    // AND ($34),Y
    // Pointer at 0x34: low=0x00, high=0x40 => base address = 0x4000
    // Y = 0x10 => final address = 0x4000 + 0x10 = 0x4010
    let cpu = run_program_with_setup(vec![0x31, 0x34, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_y = 0x10;
        cpu.memory[0x34] = 0x00; // Low byte of pointer at 0x34
        cpu.memory[0x35] = 0x40; // High byte of pointer at 0x35
        cpu.memory[0x4010] = 0x0F; // 0x4000 + 0x10
    });
    assert_eq!(cpu.register_a, 0x0A, "AND ($44),Y: 0xAA & 0x0F = 0x0A");
}

// ============================================================
// ORA - Logical Inclusive OR
// ============================================================

#[test]
fn test_ora_immediate_basic() {
    let cpu = run_with_a(0x09, 0x0F, 0xA0);
    assert_eq!(cpu.register_a, 0xAF, "ORA #$0F with A=0xA0 = 0xAF");
    assert_flag(&cpu, NEGETIVE, true, "0xAF has bit 7 set");
}

#[test]
fn test_ora_immediate_zero_result() {
    let cpu = run_with_a(0x09, 0x00, 0x00);
    assert_eq!(cpu.register_a, 0x00, "ORA #$00 with A=0x00 = 0x00");
    assert_flag(&cpu, ZERO, true, "Zero result");
}

#[test]
fn test_ora_immediate_negative() {
    let cpu = run_with_a(0x09, 0x80, 0x00);
    assert_eq!(cpu.register_a, 0x80);
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
}

#[test]
fn test_ora_zeropage() {
    let cpu = run_program_with_setup(vec![0x05, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xF0;
        cpu.memory[0x44] = 0x0F;
    });
    assert_eq!(cpu.register_a, 0xFF, "ORA $44: 0xF0 | 0x0F = 0xFF");
    assert_flag(&cpu, NEGETIVE, true, "0xFF sets NEGATIVE");
    assert_flag(&cpu, ZERO, false, "Not zero");
}

#[test]
fn test_ora_zeropagex() {
    let cpu = run_program_with_setup(vec![0x15, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xF0;
        cpu.register_x = 0x04;
        cpu.memory[0x48] = 0x0F;
    });
    assert_eq!(cpu.register_a, 0xFF);
    assert_flag(&cpu, NEGETIVE, true, "0xFF sets NEGATIVE");
}

#[test]
fn test_ora_absolute() {
    let cpu = run_program_with_setup(vec![0x0D, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.memory[0x4000] = 0x55;
    });
    assert_eq!(cpu.register_a, 0xFF);
    assert_flag(&cpu, NEGETIVE, true, "0xFF sets NEGATIVE");
}

#[test]
fn test_ora_absolutex() {
    let cpu = run_program_with_setup(vec![0x1D, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_x = 0x10;
        cpu.memory[0x4010] = 0x55;
    });
    assert_eq!(cpu.register_a, 0xFF);
}

#[test]
fn test_ora_absolutey() {
    let cpu = run_program_with_setup(vec![0x19, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_y = 0x10;
        cpu.memory[0x4010] = 0x55;
    });
    assert_eq!(cpu.register_a, 0xFF);
}

#[test]
fn test_ora_indirectx() {
    let cpu = run_program_with_setup(vec![0x01, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_x = 0x04;
        cpu.memory[0x48] = 0x00;
        cpu.memory[0x49] = 0x40;
        cpu.memory[0x4000] = 0x55;
    });
    assert_eq!(cpu.register_a, 0xFF);
}

#[test]
fn test_ora_indirecty() {
    // ORA ($44),Y
    // Pointer at 0x44: low=0x00, high=0x40 => base address = 0x4000
    // Y = 0x10 => final address = 0x4000 + 0x10 = 0x4010
    let cpu = run_program_with_setup(vec![0x11, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_y = 0x10;
        cpu.memory[0x44] = 0x00; // Low byte of pointer at 0x44
        cpu.memory[0x45] = 0x40; // High byte of pointer at 0x45
        cpu.memory[0x4010] = 0x55; // 0x4000 + 0x10
    });
    assert_eq!(cpu.register_a, 0xFF);
}

// ============================================================
// EOR - Logical Exclusive OR
// ============================================================

#[test]
fn test_eor_immediate_basic() {
    let cpu = run_with_a(0x49, 0x0F, 0xA5);
    assert_eq!(cpu.register_a, 0xAA, "EOR #$0F with A=0xA5 = 0xAA");
}

#[test]
fn test_eor_immediate_xor_self_zero() {
    let cpu = run_with_a(0x49, 0xA5, 0xA5);
    assert_eq!(cpu.register_a, 0x00, "EOR value with itself = 0");
    assert_flag(&cpu, ZERO, true, "Self-XOR is zero");
}

#[test]
fn test_eor_immediate_xor_ff() {
    let cpu = run_with_a(0x49, 0xFF, 0x55);
    assert_eq!(cpu.register_a, 0xAA, "EOR #$FF is bitwise NOT");
    assert_flag(&cpu, NEGETIVE, true, "0xAA has bit 7 set");
}

#[test]
fn test_eor_immediate_zero_xor() {
    let cpu = run_with_a(0x49, 0x00, 0x33);
    assert_eq!(cpu.register_a, 0x33, "EOR #$00 is identity");
}

#[test]
fn test_eor_zeropage() {
    let cpu = run_program_with_setup(vec![0x45, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xF0;
        cpu.memory[0x44] = 0x0F;
    });
    assert_eq!(cpu.register_a, 0xFF, "EOR $44: 0xF0 ^ 0x0F = 0xFF");
    assert_flag(&cpu, NEGETIVE, true, "0xFF sets NEGATIVE");
}

#[test]
fn test_eor_zeropagex() {
    let cpu = run_program_with_setup(vec![0x55, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_x = 0x04;
        cpu.memory[0x48] = 0x55;
    });
    assert_eq!(cpu.register_a, 0xFF);
    assert_flag(&cpu, NEGETIVE, true, "0xFF sets NEGATIVE");
}

#[test]
fn test_eor_absolute() {
    let cpu = run_program_with_setup(vec![0x4D, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.memory[0x4000] = 0x55;
    });
    assert_eq!(cpu.register_a, 0xFF);
}

#[test]
fn test_eor_absolutex() {
    let cpu = run_program_with_setup(vec![0x5D, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_x = 0x10;
        cpu.memory[0x4010] = 0x55;
    });
    assert_eq!(cpu.register_a, 0xFF);
}

#[test]
fn test_eor_absolutey() {
    let cpu = run_program_with_setup(vec![0x59, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_y = 0x10;
        cpu.memory[0x4010] = 0x55;
    });
    assert_eq!(cpu.register_a, 0xFF);
}

#[test]
fn test_eor_indirectx() {
    let cpu = run_program_with_setup(vec![0x41, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_x = 0x02;
        cpu.memory[0x46] = 0x00;
        cpu.memory[0x47] = 0x40;
        cpu.memory[0x4000] = 0x55;
    });
    assert_eq!(cpu.register_a, 0xFF);
}

#[test]
fn test_eor_indirecty() {
    // EOR ($34),Y
    // Pointer at 0x34: low=0x00, high=0x40 => base address = 0x4000
    // Y = 0x10 => final address = 0x4000 + 0x10 = 0x4010
    let cpu = run_program_with_setup(vec![0x51, 0x34, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_y = 0x10;
        cpu.memory[0x34] = 0x00; // Low byte of pointer at 0x34
        cpu.memory[0x35] = 0x40; // High byte of pointer at 0x35
        cpu.memory[0x4010] = 0x55; // 0x4000 + 0x10
    });
    assert_eq!(cpu.register_a, 0xFF);
}

// ============================================================
// ASL - Arithmetic Shift Left
// ============================================================

#[test]
fn test_asl_accumulator_basic() {
    let cpu = run_with_a(0x0a, 0x00, 0x05);
    assert_eq!(cpu.register_a, 0x0A, "ASL A: 0x05 << 1 = 0x0A");
    assert_flag(&cpu, CARRY, false, "Bit 7 was 0, no carry");
}

#[test]
fn test_asl_accumulator_carry_set() {
    let cpu = run_with_a(0x0a, 0x00, 0x80);
    assert_eq!(cpu.register_a, 0x00, "ASL A: 0x80 << 1 = 0x00");
    assert_flag(&cpu, CARRY, true, "Bit 7 was 1, carry set");
    assert_flag(&cpu, ZERO, true, "Result is zero");
}

#[test]
fn test_asl_accumulator_carry_and_negative() {
    let cpu = run_with_a(0x0a, 0x00, 0xC0);
    assert_eq!(cpu.register_a, 0x80, "ASL A: 0xC0 << 1 = 0x80");
    assert_flag(&cpu, CARRY, true, "Bit 7 was 1");
    assert_flag(&cpu, NEGETIVE, true, "Result 0x80 has bit 7 set");
}

#[test]
fn test_asl_accumulator_ff() {
    let cpu = run_with_a(0x0a, 0x00, 0xFF);
    assert_eq!(cpu.register_a, 0xFE, "ASL A: 0xFF << 1 = 0xFE");
    assert_flag(&cpu, CARRY, true, "Bit 7 was 1");
    assert_flag(&cpu, NEGETIVE, true, "0xFE has bit 7 set");
}

#[test]
fn test_asl_accumulator_01() {
    let cpu = run_with_a(0x0a, 0x00, 0x01);
    assert_eq!(cpu.register_a, 0x02);
    assert_flag(&cpu, CARRY, false, "No carry");
    assert_flag(&cpu, NEGETIVE, false, "No negative");
    assert_flag(&cpu, ZERO, false, "Not zero");
}

#[test]
fn test_asl_zeropage_basic() {
    let cpu = run_program_with_setup(vec![0x06, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x11;
    });
    assert_eq!(cpu.mem_read(0x44), 0x22, "ASL $44: 0x11 << 1 = 0x22");
    assert_flag(&cpu, CARRY, false, "Bit 7 was 0");
    assert_flag(&cpu, NEGETIVE, false, "0x22 bit 7 clear");
}

#[test]
fn test_asl_zeropage_carry() {
    let cpu = run_program_with_setup(vec![0x06, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x88;
    });
    assert_eq!(cpu.mem_read(0x44), 0x10, "ASL $44: 0x88 << 1 = 0x10");
    assert_flag(&cpu, CARRY, true, "Bit 7 was 1");
    assert_flag(&cpu, NEGETIVE, false, "0x10 bit 7 clear");
}

#[test]
fn test_asl_zeropagex_basic() {
    let cpu = run_program_with_setup(vec![0x16, 0x44, 0x00], |cpu| {
        cpu.register_x = 0x02;
        cpu.memory[0x46] = 0x20;
    });
    assert_eq!(cpu.mem_read(0x46), 0x40);
    assert_flag(&cpu, CARRY, false, "Bit 7 was 0");
}

#[test]
fn test_asl_zeropagex_wrap() {
    let cpu = run_program_with_setup(vec![0x16, 0xff, 0x00], |cpu| {
        cpu.register_x = 0x01;
        cpu.memory[0x00] = 0x10;
    });
    assert_eq!(cpu.mem_read(0x00), 0x20);
}

#[test]
fn test_asl_absolute_basic() {
    let cpu = run_program_with_setup(vec![0x0e, 0x00, 0x20, 0x00], |cpu| {
        cpu.memory[0x2000] = 0x40;
    });
    assert_eq!(cpu.mem_read(0x2000), 0x80);
    assert_flag(&cpu, CARRY, false, "Bit 7 was 0");
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
}

#[test]
fn test_asl_absolute_carry() {
    let cpu = run_program_with_setup(vec![0x0E, 0x00, 0x40, 0x00], |cpu| {
        cpu.memory[0x4000] = 0x88;
    });
    assert_eq!(cpu.mem_read(0x4000), 0x10);
    assert_flag(&cpu, CARRY, true, "Bit 7 was 1");
}

#[test]
fn test_asl_absolutex_basic() {
    let cpu = run_program_with_setup(vec![0x1e, 0x00, 0x20, 0x00], |cpu| {
        cpu.register_x = 0x10;
        cpu.memory[0x2010] = 0x20;
    });
    assert_eq!(cpu.mem_read(0x2010), 0x40);
    assert_flag(&cpu, CARRY, false, "Bit 7 was 0");
}

#[test]
fn test_asl_absolutex_carry() {
    let cpu = run_program_with_setup(vec![0x1E, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_x = 0x10;
        cpu.memory[0x4010] = 0x88;
    });
    assert_eq!(cpu.mem_read(0x4010), 0x10);
    assert_flag(&cpu, CARRY, true, "Bit 7 was 1");
}

// ============================================================
// LSR - Logical Shift Right
// ============================================================

#[test]
fn test_lsr_accumulator_basic() {
    let cpu = run_with_a(0x4a, 0x00, 0x0A);
    assert_eq!(cpu.register_a, 0x05, "LSR A: 0x0A >> 1 = 0x05");
    assert_flag(&cpu, CARRY, false, "Bit 0 was 0, no carry");
    assert_flag(&cpu, NEGETIVE, false, "MSB is 0 after LSR");
}

#[test]
fn test_lsr_accumulator_carry_set() {
    let cpu = run_with_a(0x4a, 0x00, 0x01);
    assert_eq!(cpu.register_a, 0x00, "LSR A: 0x01 >> 1 = 0x00");
    assert_flag(&cpu, CARRY, true, "Bit 0 was 1, carry set");
    assert_flag(&cpu, ZERO, true, "Result is zero");
    assert_flag(&cpu, NEGETIVE, false, "LSR clears bit 7");
}

#[test]
fn test_lsr_accumulator_negative_cleared() {
    let cpu = run_with_a(0x4a, 0x00, 0x80);
    assert_eq!(cpu.register_a, 0x40, "LSR A: 0x80 >> 1 = 0x40");
    assert_flag(&cpu, NEGETIVE, false, "LSR always clears bit 7");
    assert_flag(&cpu, CARRY, false, "Bit 0 was 0");
}

#[test]
fn test_lsr_accumulator_ff() {
    let cpu = run_with_a(0x4a, 0x00, 0xFF);
    assert_eq!(cpu.register_a, 0x7F, "LSR A: 0xFF >> 1 = 0x7F");
    assert_flag(&cpu, CARRY, true, "Bit 0 was 1");
    assert_flag(&cpu, NEGETIVE, false, "Bit 7 is 0");
}

#[test]
fn test_lsr_zeropage_basic() {
    let cpu = run_program_with_setup(vec![0x46, 0x33, 0x00], |cpu| {
        cpu.memory[0x33] = 0x08;
    });
    assert_eq!(cpu.mem_read(0x33), 0x04);
    assert_flag(&cpu, CARRY, false, "Bit 0 was 0");
    assert_flag(&cpu, NEGETIVE, false, "LSR clears bit 7");
}

#[test]
fn test_lsr_zeropage_carry() {
    let cpu = run_program_with_setup(vec![0x46, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x01;
    });
    assert_eq!(cpu.mem_read(0x44), 0x00);
    assert_flag(&cpu, CARRY, true, "Bit 0 was 1");
    assert_flag(&cpu, ZERO, true, "Result zero");
}

#[test]
fn test_lsr_zeropagex_basic() {
    let cpu = run_program_with_setup(vec![0x56, 0x44, 0x00], |cpu| {
        cpu.register_x = 0x03;
        cpu.memory[0x47] = 0x08;
    });
    assert_eq!(cpu.mem_read(0x47), 0x04, "LSR $44,X: 0x08 >> 1 = 0x04");
    assert_flag(&cpu, CARRY, false, "Bit 0 was 0");
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "LSR clears bit 7");
}

#[test]
fn test_lsr_zeropagex_wrap() {
    let cpu = run_program_with_setup(vec![0x56, 0xff, 0x00], |cpu| {
        cpu.register_x = 0x01;
        cpu.memory[0x00] = 0x10;
    });
    assert_eq!(
        cpu.mem_read(0x00),
        0x08,
        "LSR ($FF,X) with X=1 wraps to $00"
    );
}

#[test]
fn test_lsr_absolute_basic() {
    let cpu = run_program_with_setup(vec![0x4E, 0x00, 0x40, 0x00], |cpu| {
        cpu.memory[0x4000] = 0x20;
    });
    assert_eq!(cpu.mem_read(0x4000), 0x10, "LSR $4000: 0x20 >> 1 = 0x10");
    assert_flag(&cpu, CARRY, false, "Bit 0 was 0");
    assert_flag(&cpu, NEGETIVE, false, "LSR clears bit 7");
}

#[test]
fn test_lsr_absolute_carry_set() {
    let cpu = run_program_with_setup(vec![0x4E, 0x00, 0x40, 0x00], |cpu| {
        cpu.memory[0x4000] = 0x01;
    });
    assert_eq!(cpu.mem_read(0x4000), 0x00);
    assert_flag(&cpu, CARRY, true, "Bit 0 was 1");
    assert_flag(&cpu, ZERO, true, "Result zero");
}

#[test]
fn test_lsr_absolutex_basic() {
    let cpu = run_program_with_setup(vec![0x5E, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_x = 0x10;
        cpu.memory[0x4010] = 0x40;
    });
    assert_eq!(cpu.mem_read(0x4010), 0x20);
    assert_flag(&cpu, CARRY, false, "Bit 0 was 0");
}

#[test]
fn test_lsr_absolutex_wrap() {
    let cpu = run_program_with_setup(vec![0x5E, 0xFF, 0xFF, 0x00], |cpu| {
        cpu.register_x = 0x01;
        cpu.memory[0x0000] = 0x02;
    });
    assert_eq!(
        cpu.mem_read(0x0000),
        0x01,
        "LSR $FFFF,X with X=1 wraps to $0000"
    );
}

// ============================================================
// BIT - Bit Test
// ============================================================

#[test]
fn test_bit_zeropage_zero_set() {
    let cpu = run_program_with_setup(vec![0x24, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x0F;
        cpu.memory[0x44] = 0x30;
    });
    assert_flag(&cpu, ZERO, true, "0x0F & 0x30 = 0x00, ZERO set");
    assert_flag(&cpu, NEGETIVE, false, "Memory bit 7 is 0");
    assert_flag(&cpu, OVERFLOW, false, "Memory bit 6 is 0");
}

#[test]
fn test_bit_zeropage_negative_set() {
    let cpu = run_program_with_setup(vec![0x24, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xFF;
        cpu.memory[0x44] = 0x80;
    });
    assert_flag(&cpu, NEGETIVE, true, "BIT sets NEGATIVE from memory bit 7");
    assert_flag(&cpu, ZERO, false, "0xFF & 0x80 = 0x80, not zero");
    assert_flag(&cpu, OVERFLOW, false, "Memory bit 6 is 0");
}

#[test]
fn test_bit_zeropage_overflow_set() {
    let cpu = run_program_with_setup(vec![0x24, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xFF;
        cpu.memory[0x44] = 0x40;
    });
    assert_flag(&cpu, OVERFLOW, true, "BIT sets OVERFLOW from memory bit 6");
    assert_flag(&cpu, NEGETIVE, false, "Memory bit 7 is 0");
    assert_flag(&cpu, ZERO, false, "0xFF & 0x40 = 0x40, not zero");
}

#[test]
fn test_bit_zeropage_both_flags() {
    let cpu = run_program_with_setup(vec![0x24, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xFF;
        cpu.memory[0x44] = 0xC0;
    });
    assert_flag(&cpu, NEGETIVE, true, "Memory bit 7 set");
    assert_flag(&cpu, OVERFLOW, true, "Memory bit 6 set");
    assert_flag(&cpu, ZERO, false, "0xFF & 0xC0 = 0xC0, not zero");
}

#[test]
fn test_bit_absolute_zero_flag() {
    let cpu = run_program_with_setup(vec![0x2c, 0x00, 0x33, 0x00], |cpu| {
        cpu.register_a = 0x03;
        cpu.memory[0x3300] = 0xC0;
    });
    assert_flag(&cpu, ZERO, true, "0x03 & 0xC0 = 0x00, ZERO set");
}

#[test]
fn test_bit_absolute_basic() {
    let cpu = run_program_with_setup(vec![0x2c, 0x00, 0x33, 0x00], |cpu| {
        cpu.register_a = 0xFF;
        cpu.memory[0x3300] = 0xC0;
    });
    assert_flag(&cpu, NEGETIVE, true, "Memory bit 7 set");
    assert_flag(&cpu, OVERFLOW, true, "Memory bit 6 set");
    assert_flag(&cpu, ZERO, false, "0xFF & 0xC0 = 0xC0, not zero");
}

#[test]
fn test_bit_does_not_change_a() {
    let cpu = run_program_with_setup(vec![0x24, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x55;
        cpu.memory[0x44] = 0xAA;
    });
    assert_eq!(cpu.register_a, 0x55, "BIT should not modify accumulator");
}

// ============================================================
// CMP - Compare Accumulator
// ============================================================

#[test]
fn test_cmp_immediate_equal() {
    let cpu = run_program_with_setup(vec![0xc9, 0x42, 0x00], |cpu| {
        cpu.register_a = 0x42;
    });
    assert_flag(&cpu, CARRY, true, "A == M: CARRY set");
    assert_flag(&cpu, ZERO, true, "A == M: ZERO set");
    assert_flag(&cpu, NEGETIVE, false, "A - M = 0, not negative");
}

#[test]
fn test_cmp_immediate_greater() {
    let cpu = run_program_with_setup(vec![0xc9, 0x30, 0x00], |cpu| {
        cpu.register_a = 0x50;
    });
    assert_flag(&cpu, CARRY, true, "A > M: CARRY set");
    assert_flag(&cpu, ZERO, false, "A != M: ZERO clear");
    assert_flag(&cpu, NEGETIVE, false, "0x50 - 0x30 = 0x20, not negative");
}

#[test]
fn test_cmp_immediate_less() {
    let cpu = run_program_with_setup(vec![0xc9, 0x50, 0x00], |cpu| {
        cpu.register_a = 0x30;
    });
    assert_flag(&cpu, CARRY, false, "A < M: CARRY clear");
    assert_flag(&cpu, ZERO, false, "A != M: ZERO clear");
    assert_flag(
        &cpu,
        NEGETIVE,
        true,
        "0x30 - 0x50 = 0xE0 (wrapping), negative",
    );
}

#[test]
fn test_cmp_immediate_wrapping() {
    let cpu = run_program_with_setup(vec![0xc9, 0x01, 0x00], |cpu| {
        cpu.register_a = 0x00;
    });
    assert_flag(&cpu, CARRY, false, "0x00 < 0x01: CARRY clear");
    assert_flag(
        &cpu,
        NEGETIVE,
        true,
        "0x00 - 0x01 = 0xFF (wrapping), negative",
    );
}

#[test]
fn test_cmp_immediate_boundary_0x00() {
    let cpu = run_program_with_setup(vec![0xc9, 0x00, 0x00], |cpu| {
        cpu.register_a = 0x00;
    });
    assert_flag(&cpu, ZERO, true, "Equal: ZERO set");
    assert_flag(&cpu, CARRY, true, "Equal: CARRY set");
}

#[test]
fn test_cmp_immediate_boundary_0x80() {
    let cpu = run_program_with_setup(vec![0xc9, 0x80, 0x00], |cpu| {
        cpu.register_a = 0x80;
    });
    assert_flag(&cpu, ZERO, true, "Equal: ZERO set");
    assert_flag(&cpu, NEGETIVE, false, "Result is 0, not negative");
}

#[test]
fn test_cmp_immediate_boundary_0xff() {
    let cpu = run_program_with_setup(vec![0xc9, 0xFF, 0x00], |cpu| {
        cpu.register_a = 0xFF;
    });
    assert_flag(&cpu, ZERO, true, "Equal: ZERO set");
    assert_flag(&cpu, CARRY, true, "Equal: CARRY set");
}

#[test]
fn test_cmp_zeropage_basic() {
    let cpu = run_program_with_setup(vec![0xc5, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x50;
        cpu.memory[0x44] = 0x30;
    });
    assert_flag(&cpu, CARRY, true, "0x50 > 0x30: CARRY set");
}

#[test]
fn test_cmp_zeropagex_basic() {
    let cpu = run_program_with_setup(vec![0xd5, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x50;
        cpu.register_x = 0x04;
        cpu.memory[0x48] = 0x30;
    });
    assert_flag(&cpu, CARRY, true, "0x50 > 0x30: CARRY set");
}

#[test]
fn test_cmp_absolute_basic() {
    let cpu = run_program_with_setup(vec![0xcd, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0x20;
        cpu.memory[0x4000] = 0x20;
    });
    assert_flag(&cpu, CARRY, true, "Equal: CARRY set");
    assert_flag(&cpu, ZERO, true, "Equal: ZERO set");
}

#[test]
fn test_cmp_absolutex_basic() {
    let cpu = run_program_with_setup(vec![0xdd, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0x30;
        cpu.register_x = 0x10;
        cpu.memory[0x4010] = 0x10;
    });
    assert_flag(&cpu, CARRY, true, "0x30 > 0x10");
}

#[test]
fn test_cmp_absolutey_basic() {
    let cpu = run_program_with_setup(vec![0xd9, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0x30;
        cpu.register_y = 0x10;
        cpu.memory[0x4010] = 0x10;
    });
    assert_flag(&cpu, CARRY, true, "0x30 > 0x10");
}

#[test]
fn test_cmp_indirectx_basic() {
    let cpu = run_program_with_setup(vec![0xc1, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x40;
        cpu.register_x = 0x02;
        cpu.memory[0x46] = 0x00;
        cpu.memory[0x47] = 0x50;
        cpu.memory[0x5000] = 0x20;
    });
    assert_flag(&cpu, CARRY, true, "0x40 > 0x20");
}

#[test]
fn test_cmp_indirecty_basic() {
    // CMP ($44),Y
    // Pointer at 0x44: low=0x00, high=0x50 => base address = 0x5000
    // Y = 0x10 => final address = 0x5000 + 0x10 = 0x5010
    let cpu = run_program_with_setup(vec![0xd1, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x40;
        cpu.register_y = 0x10;
        cpu.memory[0x44] = 0x00; // Low byte of pointer at 0x44
        cpu.memory[0x45] = 0x50; // High byte of pointer at 0x45
        cpu.memory[0x5010] = 0x20; // 0x5000 + 0x10
    });
    assert_flag(&cpu, CARRY, true, "0x40 > 0x20");
}

// ============================================================
// CPX
// ============================================================

#[test]
fn test_cpx_immediate_equal() {
    let cpu = run_program_with_setup(vec![0xe0, 0x42, 0x00], |cpu| {
        cpu.register_x = 0x42;
    });
    assert_flag(&cpu, CARRY, true, "X == M: CARRY set");
    assert_flag(&cpu, ZERO, true, "X == M: ZERO set");
}

#[test]
fn test_cpx_immediate_less() {
    let cpu = run_program_with_setup(vec![0xe0, 0x20, 0x00], |cpu| {
        cpu.register_x = 0x10;
    });
    assert_flag(&cpu, CARRY, false, "X < M: CARRY clear");
    assert_flag(&cpu, NEGETIVE, true, "0x10 - 0x20 = 0xF0, negative");
}

#[test]
fn test_cpx_immediate_greater() {
    let cpu = run_program_with_setup(vec![0xe0, 0x10, 0x00], |cpu| {
        cpu.register_x = 0x30;
    });
    assert_flag(&cpu, CARRY, true, "X > M: CARRY set");
    assert_flag(&cpu, ZERO, false, "Not equal");
}

#[test]
fn test_cpx_zeropage_basic() {
    let cpu = run_program_with_setup(vec![0xe4, 0x44, 0x00], |cpu| {
        cpu.register_x = 0x30;
        cpu.memory[0x44] = 0x20;
    });
    assert_flag(&cpu, CARRY, true, "0x30 > 0x20");
}

#[test]
fn test_cpx_absolute_basic() {
    let cpu = run_program_with_setup(vec![0xec, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_x = 0x30;
        cpu.memory[0x4000] = 0x30;
    });
    assert_flag(&cpu, ZERO, true, "Equal: ZERO set");
    assert_flag(&cpu, CARRY, true, "Equal: CARRY set");
}

// ============================================================
// CPY
// ============================================================

#[test]
fn test_cpy_immediate_equal() {
    let cpu = run_program_with_setup(vec![0xc0, 0x42, 0x00], |cpu| {
        cpu.register_y = 0x42;
    });
    assert_flag(&cpu, CARRY, true, "Y == M: CARRY set");
    assert_flag(&cpu, ZERO, true, "Y == M: ZERO set");
}

#[test]
fn test_cpy_immediate_greater() {
    let cpu = run_program_with_setup(vec![0xc0, 0x30, 0x00], |cpu| {
        cpu.register_y = 0x50;
    });
    assert_flag(&cpu, CARRY, true, "Y > M: CARRY set");
}

#[test]
fn test_cpy_immediate_less() {
    let cpu = run_program_with_setup(vec![0xc0, 0x30, 0x00], |cpu| {
        cpu.register_y = 0x20;
    });
    assert_flag(&cpu, CARRY, false, "Y < M: CARRY clear");
    assert_flag(&cpu, NEGETIVE, true, "0x20 - 0x30 = 0xF0, negative");
}

#[test]
fn test_cpy_zeropage_basic() {
    let cpu = run_program_with_setup(vec![0xc4, 0x44, 0x00], |cpu| {
        cpu.register_y = 0x20;
        cpu.memory[0x44] = 0x30;
    });
    assert_flag(&cpu, CARRY, false, "0x20 < 0x30: CARRY clear");
}

#[test]
fn test_cpy_absolute_basic() {
    let cpu = run_program_with_setup(vec![0xcc, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_y = 0x30;
        cpu.memory[0x4000] = 0x30;
    });
    assert_flag(&cpu, ZERO, true, "Equal");
}

// ============================================================
// INC
// ============================================================

#[test]
fn test_inc_zeropage_basic() {
    let cpu = run_program_with_setup(vec![0xe6, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x10;
    });
    assert_eq!(cpu.mem_read(0x44), 0x11, "INC $44: 0x10 + 1 = 0x11");
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "0x11 bit 7 clear");
}

#[test]
fn test_inc_zeropage_zero() {
    let cpu = run_program_with_setup(vec![0xe6, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0xFF;
    });
    assert_eq!(cpu.mem_read(0x44), 0x00, "INC $44: 0xFF + 1 = 0x00");
    assert_flag(&cpu, ZERO, true, "Zero result");
    assert_flag(&cpu, NEGETIVE, false, "0x00 not negative");
}

#[test]
fn test_inc_zeropage_negative() {
    let cpu = run_program_with_setup(vec![0xe6, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x7F;
    });
    assert_eq!(cpu.mem_read(0x44), 0x80, "INC $44: 0x7F + 1 = 0x80");
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
    assert_flag(&cpu, ZERO, false, "Not zero");
}

#[test]
fn test_inc_zeropagex_basic() {
    let cpu = run_program_with_setup(vec![0xf6, 0x44, 0x00], |cpu| {
        cpu.register_x = 0x03;
        cpu.memory[0x47] = 0x20;
    });
    assert_eq!(cpu.mem_read(0x47), 0x21);
    assert_flag(&cpu, ZERO, false, "Not zero");
}

#[test]
fn test_inc_zeropagex_wrap() {
    let cpu = run_program_with_setup(vec![0xf6, 0xff, 0x00], |cpu| {
        cpu.register_x = 0x01;
        cpu.memory[0x00] = 0x50;
    });
    assert_eq!(cpu.mem_read(0x00), 0x51);
}

#[test]
fn test_inc_absolute_basic() {
    let cpu = run_program_with_setup(vec![0xee, 0x00, 0x30, 0x00], |cpu| {
        cpu.memory[0x3000] = 0x40;
    });
    assert_eq!(cpu.mem_read(0x3000), 0x41);
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "0x41 bit 7 clear");
}

#[test]
fn test_inc_absolute_negative() {
    let cpu = run_program_with_setup(vec![0xee, 0x00, 0x30, 0x00], |cpu| {
        cpu.memory[0x3000] = 0x7F;
    });
    assert_eq!(cpu.mem_read(0x3000), 0x80);
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
}

#[test]
fn test_inc_absolutex_basic() {
    let cpu = run_program_with_setup(vec![0xfe, 0x00, 0x30, 0x00], |cpu| {
        cpu.register_x = 0x10;
        cpu.memory[0x3010] = 0x20;
    });
    assert_eq!(cpu.mem_read(0x3010), 0x21);
}

// ============================================================
// DEC
// ============================================================

#[test]
fn test_dec_zeropage_basic() {
    let cpu = run_program_with_setup(vec![0xc6, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x10;
    });
    assert_eq!(cpu.mem_read(0x44), 0x0F, "DEC $44: 0x10 - 1 = 0x0F");
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "0x0F bit 7 clear");
}

#[test]
fn test_dec_zeropage_wrap_to_negative() {
    let cpu = run_program_with_setup(vec![0xc6, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x00;
    });
    assert_eq!(cpu.mem_read(0x44), 0xFF, "DEC $44: 0x00 - 1 = 0xFF");
    assert_flag(&cpu, NEGETIVE, true, "0xFF sets NEGATIVE");
    assert_flag(&cpu, ZERO, false, "Not zero");
}

#[test]
fn test_dec_zeropage_negative_flag() {
    let cpu = run_program_with_setup(vec![0xc6, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x01;
    });
    assert_eq!(cpu.mem_read(0x44), 0x00, "DEC $44: 0x01 - 1 = 0x00");
    assert_flag(&cpu, ZERO, true, "Zero result");
}

#[test]
fn test_dec_zeropagex_basic() {
    let cpu = run_program_with_setup(vec![0xd6, 0x44, 0x00], |cpu| {
        cpu.register_x = 0x04;
        cpu.memory[0x48] = 0x30;
    });
    assert_eq!(cpu.mem_read(0x48), 0x2F);
}

#[test]
fn test_dec_absolute_basic() {
    let cpu = run_program_with_setup(vec![0xce, 0x00, 0x40, 0x00], |cpu| {
        cpu.memory[0x4000] = 0x20;
    });
    assert_eq!(cpu.mem_read(0x4000), 0x1F);
    assert_flag(&cpu, ZERO, false, "Not zero");
}

#[test]
fn test_dec_absolute_negative() {
    let cpu = run_program_with_setup(vec![0xce, 0x00, 0x40, 0x00], |cpu| {
        cpu.memory[0x4000] = 0x00;
    });
    assert_eq!(cpu.mem_read(0x4000), 0xFF);
    assert_flag(&cpu, NEGETIVE, true, "0xFF sets NEGATIVE");
}

#[test]
fn test_dec_absolutex_basic() {
    let cpu = run_program_with_setup(vec![0xde, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_x = 0x20;
        cpu.memory[0x4020] = 0x10;
    });
    assert_eq!(cpu.mem_read(0x4020), 0x0F);
}

// ============================================================
// INX
// ============================================================

#[test]
fn test_inx_basic() {
    let cpu = run_program(vec![0xa2, 0x10, 0xe8, 0x00]);
    assert_eq!(cpu.register_x, 0x11, "INX: 0x10 + 1 = 0x11");
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "Not negative");
}

#[test]
fn test_inx_wrap_to_zero() {
    let cpu = run_program_with_setup(vec![0xe8, 0x00], |cpu| {
        cpu.register_x = 0xFF;
    });
    assert_eq!(cpu.register_x, 0x00, "INX: 0xFF + 1 = 0x00");
    assert_flag(&cpu, ZERO, true, "Zero result");
    assert_flag(&cpu, NEGETIVE, false, "Not negative");
}

#[test]
fn test_inx_to_negative() {
    let cpu = run_program_with_setup(vec![0xe8, 0x00], |cpu| {
        cpu.register_x = 0x7F;
    });
    assert_eq!(cpu.register_x, 0x80, "INX: 0x7F + 1 = 0x80");
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
    assert_flag(&cpu, ZERO, false, "Not zero");
}

#[test]
fn test_inx_from_zero() {
    let cpu = run_program(vec![0xa2, 0x00, 0xe8, 0x00]);
    assert_eq!(cpu.register_x, 0x01);
    assert_flag(&cpu, ZERO, false, "Not zero");
}

// ============================================================
// INY
// ============================================================

#[test]
fn test_iny_basic() {
    let cpu = run_program(vec![0xa0, 0x20, 0xc8, 0x00]);
    assert_eq!(cpu.register_y, 0x21);
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "Not negative");
}

#[test]
fn test_iny_wrap_to_zero() {
    let cpu = run_program_with_setup(vec![0xc8, 0x00], |cpu| {
        cpu.register_y = 0xFF;
    });
    assert_eq!(cpu.register_y, 0x00);
    assert_flag(&cpu, ZERO, true, "Zero result");
}

#[test]
fn test_iny_to_negative() {
    let cpu = run_program_with_setup(vec![0xc8, 0x00], |cpu| {
        cpu.register_y = 0x7F;
    });
    assert_eq!(cpu.register_y, 0x80);
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
}

// ============================================================
// DEX
// ============================================================

#[test]
fn test_dex_basic() {
    let cpu = run_program(vec![0xa2, 0x10, 0xca, 0x00]);
    assert_eq!(cpu.register_x, 0x0F);
    assert_flag(&cpu, ZERO, false, "Not zero");
}

#[test]
fn test_dex_to_zero() {
    let cpu = run_program(vec![0xa2, 0x01, 0xca, 0x00]);
    assert_eq!(cpu.register_x, 0x00);
    assert_flag(&cpu, ZERO, true, "Zero result");
}

#[test]
fn test_dex_wrap_to_negative() {
    let cpu = run_program(vec![0xa2, 0x00, 0xca, 0x00]);
    assert_eq!(cpu.register_x, 0xFF, "DEX: 0x00 - 1 = 0xFF");
    assert_flag(&cpu, NEGETIVE, true, "0xFF sets NEGATIVE");
    assert_flag(&cpu, ZERO, false, "Not zero");
}

#[test]
fn test_dex_from_negative() {
    let cpu = run_program_with_setup(vec![0xca, 0x00], |cpu| {
        cpu.register_x = 0x80;
    });
    assert_eq!(cpu.register_x, 0x7F);
    assert_flag(&cpu, NEGETIVE, false, "0x7F clears NEGATIVE");
}

// ============================================================
// DEY
// ============================================================

#[test]
fn test_dey_basic() {
    let cpu = run_program(vec![0xa0, 0x10, 0x88, 0x00]);
    assert_eq!(cpu.register_y, 0x0F);
}

#[test]
fn test_dey_to_zero() {
    let cpu = run_program(vec![0xa0, 0x01, 0x88, 0x00]);
    assert_eq!(cpu.register_y, 0x00);
    assert_flag(&cpu, ZERO, true, "Zero result");
}

#[test]
fn test_dey_wrap_to_negative() {
    let cpu = run_program(vec![0xa0, 0x00, 0x88, 0x00]);
    assert_eq!(cpu.register_y, 0xFF);
    assert_flag(&cpu, NEGETIVE, true, "0xFF sets NEGATIVE");
}

// ============================================================
// TAX
// ============================================================

#[test]
fn test_tax_basic() {
    let cpu = run_program(vec![0xa9, 0x42, 0xaa, 0x00]);
    assert_eq!(cpu.register_x, 0x42);
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "Not negative");
}

#[test]
fn test_tax_zero() {
    let cpu = run_program(vec![0xa9, 0x00, 0xaa, 0x00]);
    assert_eq!(cpu.register_x, 0x00);
    assert_flag(&cpu, ZERO, true, "Zero result");
    assert_flag(&cpu, NEGETIVE, false, "Not negative");
}

#[test]
fn test_tax_negative() {
    let cpu = run_program(vec![0xa9, 0x80, 0xaa, 0x00]);
    assert_eq!(cpu.register_x, 0x80);
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
}

// ============================================================
// BRANCH INSTRUCTIONS
// ============================================================

#[test]
fn test_bpl_branch_taken() {
    let cpu = run_program_with_setup(vec![0x10, 0x02, 0x00, 0xa9, 0x42, 0x00], |cpu| {
        cpu.register_a = 0x01; // Positive, NEGATIVE clear
    });
    assert_eq!(cpu.register_a, 0x42, "BPL should have branched to LDA");
}

#[test]
fn test_bpl_branch_not_taken() {
    let cpu = run_program_with_setup(vec![0x10, 0x02, 0xa9, 0x42, 0x00], |cpu| {
        cpu.status_register |= NEGETIVE; // Negative set, BPL not taken
    });
    // BPL not taken: PC should be at 0x8002 after BPL, which contains 0xA9 (LDA)
    // So LDA #$42 should still execute (it's the next instruction)
    assert_eq!(
        cpu.register_a, 0x42,
        "BPL not taken: should execute next instruction (LDA)"
    );
}

#[test]
fn test_bmi_branch_taken() {
    let cpu = run_program_with_setup(vec![0x30, 0x02, 0x00, 0xa9, 0x42, 0x00], |cpu| {
        cpu.status_register |= NEGETIVE; // Negative set, BMI taken
    });
    assert_eq!(cpu.register_a, 0x42, "BMI should have branched to LDA");
}

#[test]
fn test_bmi_branch_not_taken() {
    let cpu = run_program(vec![0x30, 0x02, 0xa9, 0x42, 0x00]);
    // BMI not taken (NEGATIVE clear), so next instruction LDA #$42 executes
    assert_eq!(
        cpu.register_a, 0x42,
        "BMI not taken: should execute next instruction"
    );
}

#[test]
fn test_beq_branch_taken() {
    let cpu = run_program_with_setup(vec![0xF0, 0x02, 0x00, 0xa9, 0x42, 0x00], |cpu| {
        cpu.status_register |= ZERO; // ZERO set, BEQ taken
    });
    assert_eq!(cpu.register_a, 0x42, "BEQ should have branched to LDA");
}

#[test]
fn test_beq_branch_not_taken() {
    let cpu = run_program(vec![0xF0, 0x02, 0xa9, 0x42, 0x00]);
    // BEQ not taken (ZERO clear), so next instruction LDA #$42 executes
    assert_eq!(
        cpu.register_a, 0x42,
        "BEQ not taken: should execute next instruction"
    );
}

#[test]
fn test_bne_branch_taken() {
    let cpu = run_program(vec![0xD0, 0x02, 0x00, 0xa9, 0x42, 0x00]);
    // BNE taken (ZERO clear)
    assert_eq!(cpu.register_a, 0x42, "BNE should have branched to LDA");
}

#[test]
fn test_bne_branch_not_taken() {
    let cpu = run_program_with_setup(vec![0xD0, 0x02, 0xa9, 0x42, 0x00], |cpu| {
        cpu.status_register |= ZERO; // ZERO set, BNE not taken
    });
    // BNE not taken, so next instruction LDA #$42 executes
    assert_eq!(
        cpu.register_a, 0x42,
        "BNE not taken: should execute next instruction"
    );
}

#[test]
fn test_bcs_branch_taken() {
    let cpu = run_program_with_setup(vec![0xB0, 0x02, 0x00, 0xa9, 0x42, 0x00], |cpu| {
        cpu.status_register |= CARRY; // CARRY set, BCS taken
    });
    assert_eq!(cpu.register_a, 0x42, "BCS should have branched to LDA");
}

#[test]
fn test_bcs_branch_not_taken() {
    let cpu = run_program(vec![0xB0, 0x02, 0xa9, 0x42, 0x00]);
    // BCS not taken (CARRY clear), so next instruction LDA #$42 executes
    assert_eq!(
        cpu.register_a, 0x42,
        "BCS not taken: should execute next instruction"
    );
}

#[test]
fn test_bcc_branch_taken() {
    let cpu = run_program(vec![0x90, 0x02, 0x00, 0xa9, 0x42, 0x00]);
    // BCC taken (CARRY clear)
    assert_eq!(cpu.register_a, 0x42, "BCC should have branched to LDA");
}

#[test]
fn test_bcc_branch_not_taken() {
    let cpu = run_program_with_setup(vec![0x90, 0x02, 0xa9, 0x42, 0x00], |cpu| {
        cpu.status_register |= CARRY; // CARRY set, BCC not taken
    });
    // BCC not taken, so next instruction LDA #$42 executes
    assert_eq!(
        cpu.register_a, 0x42,
        "BCC not taken: should execute next instruction"
    );
}

#[test]
fn test_bvs_branch_taken() {
    let cpu = run_program_with_setup(vec![0x70, 0x02, 0x00, 0xa9, 0x42, 0x00], |cpu| {
        cpu.status_register |= OVERFLOW; // OVERFLOW set, BVS taken
    });
    assert_eq!(cpu.register_a, 0x42, "BVS should have branched to LDA");
}

#[test]
fn test_bvc_branch_taken() {
    let cpu = run_program(vec![0x50, 0x02, 0x00, 0xa9, 0x42, 0x00]);
    // BVC taken (OVERFLOW clear)
    assert_eq!(cpu.register_a, 0x42, "BVC should have branched to LDA");
}

// ============================================================
// JMP
// ============================================================

#[test]
fn test_jmp_absolute() {
    let mut cpu = CPU::new();
    cpu.memory[0x1234] = 0x45;
    cpu.memory[0x1235] = 0x23;
    cpu.load_program(vec![0x4c, 0x34, 0x12, 0x00]);
    cpu.reset();
    cpu.interpret();
    assert_eq!(
        cpu.program_counter, 0x2346,
        "JMP should set PC to address, then BRK increments"
    );
}

#[test]
fn test_jmp_indirect() {
    let mut cpu = CPU::new();
    cpu.memory[0x34] = 0x00;
    cpu.memory[0x35] = 0x40;
    cpu.memory[0x4000] = 0x00;
    cpu.memory[0x4001] = 0x40;
    cpu.load_program(vec![0x6c, 0x34, 0x00]);
    cpu.reset();
    cpu.interpret();
    assert_eq!(
        cpu.program_counter, 0x4001,
        "JMP indirect should set PC to indirect address"
    );
}

// ============================================================
// JSR / RTS
// ============================================================

#[test]
fn test_jsr_basic() {
    let mut cpu = CPU::new();
    cpu.memory[0x8100] = 0x60;
    cpu.load_program(vec![0x20, 0x00, 0x81, 0xa9, 0x42, 0x00]);
    cpu.reset();
    cpu.interpret();
    assert_eq!(
        cpu.register_a, 0x42,
        "After JSR/RTS, should execute LDA #$42"
    );
}

// ============================================================
// FLAG INSTRUCTIONS
// ============================================================

#[test]
fn test_clc_clears_carry() {
    let mut cpu = CPU::new();
    cpu.status_register |= CARRY;
    cpu.load_program(vec![0x18, 0x00]);
    cpu.reset();
    // Need to set CARRY after reset
    cpu.status_register |= CARRY;
    cpu.interpret();
    assert_flag(&cpu, CARRY, false, "CLC should clear CARRY");
}

#[test]
fn test_cli_clears_interrupt() {
    let mut cpu = CPU::new();
    cpu.load_program(vec![0x58, 0x00]);
    cpu.reset();
    cpu.status_register |= INTERRUPT;
    cpu.interpret();
    assert_flag(&cpu, INTERRUPT, false, "CLI should clear INTERRUPT");
}

#[test]
fn test_clv_clears_overflow() {
    let mut cpu = CPU::new();
    cpu.load_program(vec![0xB8, 0x00]);
    cpu.reset();
    cpu.status_register |= OVERFLOW;
    cpu.interpret();
    assert_flag(&cpu, OVERFLOW, false, "CLV should clear OVERFLOW");
}

// ============================================================
// NOP
// ============================================================

#[test]
fn test_nop() {
    let cpu = run_program(vec![0xea, 0xea, 0xa9, 0x42, 0x00]);
    assert_eq!(cpu.register_a, 0x42, "NOPs should not affect A");
}

// ============================================================
// INSTRUCTION CHAINS
// ============================================================

#[test]
fn test_chain_lda_tax_inx_cmp() {
    let cpu = run_program(vec![0xa9, 0x10, 0xaa, 0xe8, 0xe0, 0x11, 0x00]);
    assert_eq!(cpu.register_x, 0x11, "LDA #$10, TAX, INX -> X = 0x11");
    // After CMP #$11, ZERO should be set
    assert_flag(&cpu, ZERO, true, "X == 0x11, CMP #$11 should set ZERO");
    assert_flag(&cpu, CARRY, true, "X >= 0x11, should set CARRY");
}

#[test]
fn test_chain_shift_flags() {
    let cpu = run_program(vec![0xa9, 0x01, 0x0a, 0x0a, 0x0a, 0x00]);
    assert_eq!(cpu.register_a, 0x08, "0x01 ASL 3 times = 0x08");
    assert_flag(&cpu, CARRY, false, "Last ASL: bit 7 was 0");
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "Not negative");
}

// ============================================================
// ADC - Add with Carry
// ============================================================

#[test]
fn test_adc_immediate_basic() {
    // A=0x30, ADC #0x10 with CARRY set => 0x30 + 0x10 + 1 = 0x41
    let cpu = run_program_with_setup(vec![0x69, 0x10, 0x00], |cpu| {
        cpu.register_a = 0x30;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x41, "ADC #$10 + A=0x30 + C=1 = 0x41");
    assert_flag(&cpu, CARRY, false, "0x41 <= 0xFF: no carry");
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "0x41 not negative");
}

#[test]
fn test_adc_immediate_no_carry_flag() {
    // A=0x30, ADC #0x10 with CARRY clear => 0x30 + 0x10 + 0 = 0x40
    let cpu = run_program_with_setup(vec![0x69, 0x10, 0x00], |cpu| {
        cpu.register_a = 0x30;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.register_a, 0x40, "ADC #$10 + A=0x30 + C=0 = 0x40");
    assert_flag(&cpu, CARRY, false, "0x40 <= 0xFF: no carry");
}

#[test]
fn test_adc_immediate_carry_set() {
    // A=0x80, ADC #0x80 with CARRY clear => 0x80 + 0x80 = 0x100, result=0x00, CARRY set
    let cpu = run_program_with_setup(vec![0x69, 0x80, 0x00], |cpu| {
        cpu.register_a = 0x80;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.register_a, 0x00, "ADC #$80 + A=0x80 = 0x00 with carry");
    assert_flag(&cpu, CARRY, true, "Result > 0xFF: carry set");
    assert_flag(&cpu, ZERO, true, "Zero result");
}

#[test]
fn test_adc_immediate_overflow_positive() {
    // A=0x50, ADC #0x30 with CARRY clear => 0x50 + 0x30 = 0x80
    // Positive + Positive = Negative => OVERFLOW
    let cpu = run_program_with_setup(vec![0x69, 0x30, 0x00], |cpu| {
        cpu.register_a = 0x50;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.register_a, 0x80, "ADC #$30 + A=0x50 = 0x80");
    assert_flag(&cpu, OVERFLOW, true, "Pos + Pos = Neg: OVERFLOW");
    assert_flag(&cpu, NEGETIVE, true, "0x80 is negative");
}

#[test]
fn test_adc_immediate_overflow_negative() {
    // A=0xD0, ADC #0xB0 with CARRY clear => 0xD0 + 0xB0 = 0x180, result=0x80
    // Negative (0xD0) + Negative (0xB0) = Negative (0x80) => NO OVERFLOW
    // Overflow occurs when result sign differs from both operands
    let cpu = run_program_with_setup(vec![0x69, 0xB0, 0x00], |cpu| {
        cpu.register_a = 0xD0;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.register_a, 0x80, "ADC #$B0 + A=0xD0 = 0x80");
    assert_flag(&cpu, OVERFLOW, false, "Neg + Neg = Neg: NO OVERFLOW");
    assert_flag(&cpu, CARRY, true, "Result > 0xFF: carry set");
    assert_flag(&cpu, NEGETIVE, true, "Result 0x80 is negative");
}

#[test]
fn test_adc_zeropage() {
    let cpu = run_program_with_setup(vec![0x65, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x20;
        cpu.memory[0x44] = 0x10;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x31, "ADC $44: 0x20 + 0x10 + 1 = 0x31");
}

#[test]
fn test_adc_zeropagex() {
    let cpu = run_program_with_setup(vec![0x75, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x20;
        cpu.register_x = 0x04;
        cpu.memory[0x48] = 0x10;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x31);
}

#[test]
fn test_adc_absolute() {
    let cpu = run_program_with_setup(vec![0x6D, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0x20;
        cpu.memory[0x4000] = 0x10;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x31);
}

#[test]
fn test_adc_absolutex() {
    let cpu = run_program_with_setup(vec![0x7D, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0x20;
        cpu.register_x = 0x10;
        cpu.memory[0x4010] = 0x10;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x31);
}

#[test]
fn test_adc_absolutey() {
    let cpu = run_program_with_setup(vec![0x79, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0x20;
        cpu.register_y = 0x10;
        cpu.memory[0x4010] = 0x10;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x31);
}

#[test]
fn test_adc_indirectx() {
    let cpu = run_program_with_setup(vec![0x61, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x20;
        cpu.register_x = 0x02;
        cpu.memory[0x46] = 0x00;
        cpu.memory[0x47] = 0x40;
        cpu.memory[0x4000] = 0x10;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x31);
}

#[test]
fn test_adc_indirecty() {
    // ADC ($44),Y
    // Pointer at 0x44: low=0x00, high=0x40 => base=0x4000
    // Y=0x10 => final=0x4010
    let cpu = run_program_with_setup(vec![0x71, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x20;
        cpu.register_y = 0x10;
        cpu.memory[0x44] = 0x00;
        cpu.memory[0x45] = 0x40;
        cpu.memory[0x4010] = 0x10;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x31);
}

// ============================================================
// SBC - Subtract with Carry
// ============================================================

// ============================================================
// SBC - Subtract with Carry
// ============================================================

#[test]
fn test_sbc_immediate_basic() {
    // A=0x50, SBC #0x20 with CARRY set (no borrow)
    // 0x50 + 0xDF + 1 = 0x130, result=0x30, CARRY set
    let cpu = run_program_with_setup(vec![0xe9, 0x20, 0x00], |cpu| {
        cpu.register_a = 0x50;
        cpu.status_register |= CARRY; // No borrow
    });
    assert_eq!(cpu.register_a, 0x30, "SBC #$20 from 0x50 = 0x30");
    assert_flag(&cpu, CARRY, true, "Result <= 0xFF: CARRY set (no borrow)");
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "0x30 not negative");
}

#[test]
fn test_sbc_immediate_with_borrow() {
    // A=0x50, SBC #0x60 with CARRY clear (borrow)
    // 0x50 + 0x9F + 0 = 0xEF, result=0xEF, CARRY clear
    let cpu = run_program_with_setup(vec![0xe9, 0x60, 0x00], |cpu| {
        cpu.register_a = 0x50;
        cpu.status_register &= !CARRY; // Borrow active
    });
    assert_eq!(cpu.register_a, 0xEF, "SBC #$60 from 0x50 with borrow = 0xEF");
    assert_flag(&cpu, CARRY, false, "Result > 0xFF wraps: CARRY clear (borrow)");
    assert_flag(&cpu, NEGETIVE, true, "0xEF is negative");
}

#[test]
fn test_sbc_immediate_zero_result() {
    // A=0x42, SBC #0x42 with CARRY set
    // 0x42 + 0xBD + 1 = 0x100, result=0x00, CARRY set
    let cpu = run_program_with_setup(vec![0xe9, 0x42, 0x00], |cpu| {
        cpu.register_a = 0x42;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x00, "SBC #$42 from 0x42 = 0x00");
    assert_flag(&cpu, ZERO, true, "Zero result");
    assert_flag(&cpu, CARRY, true, "Result exact: CARRY set");
}

#[test]
fn test_sbc_immediate_overflow_positive_to_negative() {
    // A=0x50, SBC #0xD0 with CARRY set
    // 0x50 + 0x2F + 1 = 0x80, result=0x80, OVERFLOW set (pos - neg = neg)
    let cpu = run_program_with_setup(vec![0xe9, 0xD0, 0x00], |cpu| {
        cpu.register_a = 0x50;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x80, "SBC #$D0 from 0x50 = 0x80");
    assert_flag(&cpu, OVERFLOW, true, "Positive - Negative = Negative: OVERFLOW");
    assert_flag(&cpu, NEGETIVE, true, "0x80 is negative");
}

#[test]
fn test_sbc_zeropage() {
    let cpu = run_program_with_setup(vec![0xe5, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x80;
        cpu.memory[0x44] = 0x40;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x40, "SBC $44: 0x80 - 0x40 = 0x40");
    assert_flag(&cpu, CARRY, true, "No borrow");
}

#[test]
fn test_sbc_zeropagex() {
    let cpu = run_program_with_setup(vec![0xf5, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x80;
        cpu.register_x = 0x04;
        cpu.memory[0x48] = 0x40;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x40);
}

#[test]
fn test_sbc_absolute() {
    let cpu = run_program_with_setup(vec![0xed, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0xFF;
        cpu.memory[0x4000] = 0x01;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0xFE, "SBC $4000: 0xFF - 0x01 = 0xFE");
    assert_flag(&cpu, CARRY, true, "No borrow");
    assert_flag(&cpu, NEGETIVE, true, "0xFE is negative");
}

#[test]
fn test_sbc_absolutex() {
    let cpu = run_program_with_setup(vec![0xfd, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0x80;
        cpu.register_x = 0x10;
        cpu.memory[0x4010] = 0x40;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x40);
}

#[test]
fn test_sbc_absolutey() {
    let cpu = run_program_with_setup(vec![0xf9, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0x80;
        cpu.register_y = 0x10;
        cpu.memory[0x4010] = 0x40;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x40);
}

#[test]
fn test_sbc_indirectx() {
    let cpu = run_program_with_setup(vec![0xe1, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x80;
        cpu.register_x = 0x02;
        cpu.memory[0x46] = 0x00;
        cpu.memory[0x47] = 0x40;
        cpu.memory[0x4000] = 0x40;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x40);
}

#[test]
fn test_sbc_indirecty() {
    // SBC ($44),Y
    // Pointer at 0x44: low=0x00, high=0x40 => base=0x4000
    // Y=0x10 => final=0x4010
    let cpu = run_program_with_setup(vec![0xf1, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x80;
        cpu.register_y = 0x10;
        cpu.memory[0x44] = 0x00;
        cpu.memory[0x45] = 0x40;
        cpu.memory[0x4010] = 0x40;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x40);
}

// ============================================================
// STA - Store Accumulator
// ============================================================

#[test]
fn test_sta_zeropage() {
    let cpu = run_program_with_setup(vec![0x85, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x42;
    });
    assert_eq!(cpu.mem_read(0x44), 0x42, "STA $44 should store A=0x42");
}

#[test]
fn test_sta_zeropagex() {
    let cpu = run_program_with_setup(vec![0x95, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x42;
        cpu.register_x = 0x03;
    });
    assert_eq!(cpu.mem_read(0x47), 0x42, "STA $44,X should store at 0x47");
}

#[test]
fn test_sta_zeropagex_wrap() {
    let cpu = run_program_with_setup(vec![0x95, 0xff, 0x00], |cpu| {
        cpu.register_a = 0x99;
        cpu.register_x = 0x01;
    });
    assert_eq!(cpu.mem_read(0x00), 0x99, "STA $FF,X with X=1 wraps to $00");
}

#[test]
fn test_sta_absolute() {
    let cpu = run_program_with_setup(vec![0x8d, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0x55;
    });
    assert_eq!(cpu.mem_read(0x4000), 0x55, "STA $4000 should store A=0x55");
}

#[test]
fn test_sta_absolutex() {
    let cpu = run_program_with_setup(vec![0x9d, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0x66;
        cpu.register_x = 0x10;
    });
    assert_eq!(cpu.mem_read(0x4010), 0x66, "STA $4000,X should store at 0x4010");
}

#[test]
fn test_sta_absolutey() {
    let cpu = run_program_with_setup(vec![0x99, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_a = 0x77;
        cpu.register_y = 0x10;
    });
    assert_eq!(cpu.mem_read(0x4010), 0x77, "STA $4000,Y should store at 0x4010");
}

#[test]
fn test_sta_indirectx() {
    // STA ($44,X)
    // X=0x02, ptr=0x46 => low=0x00, high=0x40 => address=0x4000
    let cpu = run_program_with_setup(vec![0x81, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_x = 0x02;
        cpu.memory[0x46] = 0x00;
        cpu.memory[0x47] = 0x40;
    });
    assert_eq!(cpu.mem_read(0x4000), 0xAA, "STA ($44,X) should store at 0x4000");
}

#[test]
fn test_sta_indirecty() {
    // STA ($44),Y
    // ptr=0x44: low=0x00, high=0x40 => base=0x4000, Y=0x10 => final=0x4010
    let cpu = run_program_with_setup(vec![0x91, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xBB;
        cpu.register_y = 0x10;
        cpu.memory[0x44] = 0x00;
        cpu.memory[0x45] = 0x40;
    });
    assert_eq!(cpu.mem_read(0x4010), 0xBB, "STA ($44),Y should store at 0x4010");
}

// ============================================================
// STX - Store X Register
// ============================================================

#[test]
fn test_stx_zeropage() {
    let cpu = run_program_with_setup(vec![0x86, 0x44, 0x00], |cpu| {
        cpu.register_x = 0x33;
    });
    assert_eq!(cpu.mem_read(0x44), 0x33, "STX $44 should store X=0x33");
}

#[test]
fn test_stx_zeropagey() {
    let cpu = run_program_with_setup(vec![0x96, 0x44, 0x00], |cpu| {
        cpu.register_x = 0x44;
        cpu.register_y = 0x03;
    });
    assert_eq!(cpu.mem_read(0x47), 0x44, "STX $44,Y should store at 0x47");
}

#[test]
fn test_stx_zeropagey_wrap() {
    let cpu = run_program_with_setup(vec![0x96, 0xff, 0x00], |cpu| {
        cpu.register_x = 0xAA;
        cpu.register_y = 0x01;
    });
    assert_eq!(cpu.mem_read(0x00), 0xAA, "STX $FF,Y with Y=1 wraps to $00");
}

#[test]
fn test_stx_absolute() {
    let cpu = run_program_with_setup(vec![0x8e, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_x = 0x55;
    });
    assert_eq!(cpu.mem_read(0x4000), 0x55, "STX $4000 should store X=0x55");
}

// ============================================================
// STY - Store Y Register
// ============================================================

#[test]
fn test_sty_zeropage() {
    let cpu = run_program_with_setup(vec![0x84, 0x44, 0x00], |cpu| {
        cpu.register_y = 0x33;
    });
    assert_eq!(cpu.mem_read(0x44), 0x33, "STY $44 should store Y=0x33");
}

#[test]
fn test_sty_zeropagex() {
    // STY $44,Y - uses ZeroPageY addressing (adds Y to base address)
    // This is actually STY ZeroPageY, not ZeroPageX as the test name suggests
    let cpu = run_program_with_setup(vec![0x94, 0x44, 0x00], |cpu| {
        cpu.register_y = 0x44;
        cpu.register_x = 0x03; // X is not used in ZeroPageY mode
    });
    // ZeroPageY: address = 0x44 + Y = 0x44 + 0x44 = 0x88
    assert_eq!(cpu.mem_read(0x88), 0x44, "STY $44,Y should store Y at 0x88 (0x44 + 0x44)");
}

#[test]
fn test_sty_zeropagex_wrap() {
    // STY $FF,Y - ZeroPageY with wrap
    let cpu = run_program_with_setup(vec![0x94, 0xff, 0x00], |cpu| {
        cpu.register_y = 0xAA;
        cpu.register_x = 0x01; // Not used
    });
    // ZeroPageY: address = 0xFF + 0xAA = 0xA9 (wraps in zero page)
    assert_eq!(cpu.mem_read(0xa9), 0xAA, "STY $FF,Y wraps in zero page");
}

#[test]
fn test_sty_absolute() {
    let cpu = run_program_with_setup(vec![0x8c, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_y = 0x55;
    });
    assert_eq!(cpu.mem_read(0x4000), 0x55, "STY $4000 should store Y=0x55");
}

// ============================================================
// STA, STX, STY - Register isolation
// ============================================================

#[test]
fn test_sta_does_not_affect_x_y() {
    let cpu = run_program_with_setup(vec![0x85, 0x44, 0x00], |cpu| {
        cpu.register_a = 0x42;
        cpu.register_x = 0x33;
        cpu.register_y = 0x44;
    });
    assert_eq!(cpu.register_x, 0x33, "STA should not modify X");
    assert_eq!(cpu.register_y, 0x44, "STA should not modify Y");
}

// ============================================================
// ROL - Rotate Left
// ============================================================

#[test]
fn test_rol_accumulator_basic() {
    // A=0x55, C=0 => ROL A = 0xAA, C=0
    let cpu = run_program_with_setup(vec![0x2a, 0x00], |cpu| {
        cpu.register_a = 0x55;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.register_a, 0xAA, "ROL A: 0x55 << 1 = 0xAA");
    assert_flag(&cpu, CARRY, false, "Bit 7 was 0: no carry");
}

#[test]
fn test_rol_accumulator_carry_from_bit7() {
    // A=0x80, C=0 => ROL A = 0x00, C=1
    let cpu = run_program_with_setup(vec![0x2a, 0x00], |cpu| {
        cpu.register_a = 0x80;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.register_a, 0x00, "ROL A: 0x80 << 1 = 0x00");
    assert_flag(&cpu, CARRY, true, "Bit 7 was 1: carry set");
    assert_flag(&cpu, ZERO, true, "Result is zero");
}

#[test]
fn test_rol_accumulator_carry_into_bit0() {
    // A=0x55, C=1 => ROL A = 0xAB, C=0
    let cpu = run_program_with_setup(vec![0x2a, 0x00], |cpu| {
        cpu.register_a = 0x55;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0xAB, "ROL A: 0x55 << 1 | 1 = 0xAB");
    assert_flag(&cpu, CARRY, false, "Bit 7 was 0: no carry");
}

#[test]
fn test_rol_accumulator_carry_bit7_and_carry_flag() {
    // A=0x80, C=1 => ROL A = 0x01, C=1
    let cpu = run_program_with_setup(vec![0x2a, 0x00], |cpu| {
        cpu.register_a = 0x80;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x01, "ROL A: 0x80 << 1 | 1 = 0x01");
    assert_flag(&cpu, CARRY, true, "Bit 7 was 1: carry set");
}

#[test]
fn test_rol_zeropage() {
    let cpu = run_program_with_setup(vec![0x26, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x40;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x44), 0x80, "ROL $44: 0x40 << 1 = 0x80");
    assert_flag(&cpu, CARRY, false, "Bit 7 was 0: no carry");
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
}

#[test]
fn test_rol_zeropage_carry() {
    let cpu = run_program_with_setup(vec![0x26, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x88;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x44), 0x10, "ROL $44: 0x88 << 1 = 0x10");
    assert_flag(&cpu, CARRY, true, "Bit 7 was 1: carry set");
}

#[test]
fn test_rol_zeropagex() {
    let cpu = run_program_with_setup(vec![0x36, 0x44, 0x00], |cpu| {
        cpu.register_x = 0x02;
        cpu.memory[0x46] = 0x20;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x46), 0x40);
}

#[test]
fn test_rol_zeropagex_wrap() {
    let cpu = run_program_with_setup(vec![0x36, 0xff, 0x00], |cpu| {
        cpu.register_x = 0x01;
        cpu.memory[0x00] = 0x10;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x00), 0x20);
}

#[test]
fn test_rol_absolute() {
    let cpu = run_program_with_setup(vec![0x2e, 0x00, 0x20, 0x00], |cpu| {
        cpu.memory[0x2000] = 0x40;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x2000), 0x80);
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
}

#[test]
fn test_rol_absolutex() {
    let cpu = run_program_with_setup(vec![0x3e, 0x00, 0x20, 0x00], |cpu| {
        cpu.register_x = 0x10;
        cpu.memory[0x2010] = 0x20;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x2010), 0x40);
}

// ============================================================
// ROR - Rotate Right
// ============================================================

#[test]
fn test_ror_accumulator_basic() {
    // A=0xAA, C=0 => ROR A = 0x55, C=0
    let cpu = run_program_with_setup(vec![0x6a, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.register_a, 0x55, "ROR A: 0xAA >> 1 = 0x55");
    assert_flag(&cpu, CARRY, false, "Bit 0 was 0: no carry");
    assert_flag(&cpu, NEGETIVE, false, "MSB is 0");
}

#[test]
fn test_ror_accumulator_carry_from_bit0() {
    // A=0x01, C=0 => ROR A = 0x00, C=1
    let cpu = run_program_with_setup(vec![0x6a, 0x00], |cpu| {
        cpu.register_a = 0x01;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.register_a, 0x00, "ROR A: 0x01 >> 1 = 0x00");
    assert_flag(&cpu, CARRY, true, "Bit 0 was 1: carry set");
    assert_flag(&cpu, ZERO, true, "Result is zero");
}

#[test]
fn test_ror_accumulator_carry_into_bit7() {
    // A=0xAA, C=1 => ROR A = 0xD5, C=0
    let cpu = run_program_with_setup(vec![0x6a, 0x00], |cpu| {
        cpu.register_a = 0xAA;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0xD5, "ROR A: 0xAA >> 1 | 0x80 = 0xD5");
    assert_flag(&cpu, CARRY, false, "Bit 0 was 0: no carry");
    assert_flag(&cpu, NEGETIVE, true, "Bit 7 set from carry");
}

#[test]
fn test_ror_accumulator_carry_bit0_and_carry_flag() {
    // A=0x01, C=1 => ROR A = 0x80, C=1
    let cpu = run_program_with_setup(vec![0x6a, 0x00], |cpu| {
        cpu.register_a = 0x01;
        cpu.status_register |= CARRY;
    });
    assert_eq!(cpu.register_a, 0x80, "ROR A: 0x01 >> 1 | 0x80 = 0x80");
    assert_flag(&cpu, CARRY, true, "Bit 0 was 1: carry set");
    assert_flag(&cpu, NEGETIVE, true, "Bit 7 set from carry");
}

#[test]
fn test_ror_zeropage() {
    let cpu = run_program_with_setup(vec![0x66, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x08;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x44), 0x04, "ROR $44: 0x08 >> 1 = 0x04");
    assert_flag(&cpu, CARRY, false, "Bit 0 was 0: no carry");
}

#[test]
fn test_ror_zeropage_carry() {
    let cpu = run_program_with_setup(vec![0x66, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x01;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x44), 0x00, "ROR $44: 0x01 >> 1 = 0x00");
    assert_flag(&cpu, CARRY, true, "Bit 0 was 1: carry set");
    assert_flag(&cpu, ZERO, true, "Result is zero");
}

#[test]
fn test_ror_zeropagex() {
    let cpu = run_program_with_setup(vec![0x76, 0x44, 0x00], |cpu| {
        cpu.register_x = 0x03;
        cpu.memory[0x47] = 0x08;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x47), 0x04);
}

#[test]
fn test_ror_zeropagex_wrap() {
    let cpu = run_program_with_setup(vec![0x76, 0xff, 0x00], |cpu| {
        cpu.register_x = 0x01;
        cpu.memory[0x00] = 0x10;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x00), 0x08);
}

#[test]
fn test_ror_absolute() {
    let cpu = run_program_with_setup(vec![0x6e, 0x00, 0x40, 0x00], |cpu| {
        cpu.memory[0x4000] = 0x20;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x4000), 0x10);
}

#[test]
fn test_ror_absolutex() {
    let cpu = run_program_with_setup(vec![0x7e, 0x00, 0x40, 0x00], |cpu| {
        cpu.register_x = 0x10;
        cpu.memory[0x4010] = 0x40;
        cpu.status_register &= !CARRY;
    });
    assert_eq!(cpu.mem_read(0x4010), 0x20);
}

// ============================================================
// STACK OPERATIONS
// ============================================================

#[test]
fn test_pha_pushes_accumulator() {
    // PHA pushes A onto stack at 0x0100 + SP (initial SP=0xFF, after PHA SP=0xFE)
    // Value stored at 0x01FE and 0x01FF (stack grows downward)
    let cpu = run_program_with_setup(vec![0x48, 0x00], |cpu| {
        cpu.register_a = 0x42;
    });
    // After PHA: SP decrements from 0xFF to 0xFE, stores A at 0x01FE
    assert_eq!(cpu.stack_pointer, 0xFE, "PHA should decrement SP by 1");
    assert_eq!(cpu.mem_read(0x01FE), 0x42, "PHA should store A at 0x01FE (SP=0xFE)");
}

#[test]
fn test_pha_then_pla() {
    let cpu = run_program(vec![0xa9, 0x42, 0x48, 0x68, 0x00]);
    assert_eq!(cpu.register_a, 0x42, "PLA should recover value pushed by PHA");
    assert_eq!(cpu.stack_pointer, 0xFF, "After PHA+PLA, SP should be back to 0xFF");
}

#[test]
fn test_php_pushes_status() {
    let cpu = run_program_with_setup(vec![0x08, 0x00], |cpu| {
        cpu.status_register = 0x42;
    });
    assert_eq!(cpu.stack_pointer, 0xFE, "PHP should decrement SP by 1");
    assert_eq!(cpu.mem_read(0x01FE), 0x42, "PHP should store status at 0x01FE");
}

#[test]
fn test_php_then_plp() {
    let cpu = run_program_with_setup(vec![0x08, 0x28, 0x00], |cpu| {
        cpu.status_register = 0x42;
    });
    assert_eq!(cpu.status_register, 0x42, "PLP should recover status pushed by PHP");
    assert_eq!(cpu.stack_pointer, 0xFF, "After PHP+PLP, SP should be back to 0xFF");
}

#[test]
fn test_rti_returns_from_interrupt() {
    // RTI pops status then PC (2 bytes)
    // After RTI, SP should increase by 3 (from 0xFD to 0x00)
    // Actually: SP starts at 0xFF, pushes 3 bytes => SP=0xFC, then RTI pops 3 => SP=0xFF
    let mut cpu = CPU::new();
    // Set up stack manually: push PC (0x1234) and status (0x42)
    cpu.stack_push(0x1234);
    cpu.stack_push_u8(0x42);
    // Now SP should be 0xFD (0xFF - 2 for u16, -1 for u8 = 0xFC... let me check)
    // Actually: start SP=0xFF, push u16 => SP=0xFD, push u8 => SP=0xFC
    // So RTI should pop u8 then u16, ending at SP=0xFF

    // Load RTI instruction
    cpu.load_program(vec![0x40, 0x00]);
    cpu.program_counter = 0x8000; // Point to RTI
    cpu.interpret();

    assert_eq!(cpu.program_counter, 0x1235, "RTI should restore PC to 0x1234+1");
    assert_eq!(cpu.status_register, 0x42, "RTI should restore status to 0x42");
    assert_eq!(cpu.stack_pointer, 0xFF, "After RTI, SP should be 0xFF");
}

// ============================================================
// FLAG SET INSTRUCTIONS
// ============================================================

#[test]
fn test_sec_sets_carry() {
    let cpu = run_program_with_setup(vec![0x38, 0x00], |cpu| {
        cpu.status_register &= !CARRY;
    });
    assert_flag(&cpu, CARRY, true, "SEC should set CARRY");
}

#[test]
fn test_sed_sets_decimal() {
    let cpu = run_program_with_setup(vec![0xf8, 0x00], |cpu| {
        cpu.status_register &= !DECIMAL;
    });
    assert_flag(&cpu, DECIMAL, true, "SED should set DECIMAL");
}

#[test]
fn test_sei_sets_interrupt() {
    let cpu = run_program_with_setup(vec![0x78, 0x00], |cpu| {
        cpu.status_register &= !INTERRUPT;
    });
    assert_flag(&cpu, INTERRUPT, true, "SEI should set INTERRUPT");
}

// ============================================================
// TRANSFER INSTRUCTIONS
// ============================================================

#[test]
fn test_tay_basic() {
    let cpu = run_program(vec![0xa9, 0x42, 0xa8, 0x00]);
    assert_eq!(cpu.register_y, 0x42, "TAY should transfer A to Y");
    assert_flag(&cpu, ZERO, false, "Not zero");
    assert_flag(&cpu, NEGETIVE, false, "Not negative");
}

#[test]
fn test_tay_zero() {
    let cpu = run_program(vec![0xa9, 0x00, 0xa8, 0x00]);
    assert_eq!(cpu.register_y, 0x00, "TAY with A=0 should set Y=0");
    assert_flag(&cpu, ZERO, true, "Zero result");
}

#[test]
fn test_tay_negative() {
    let cpu = run_program(vec![0xa9, 0x80, 0xa8, 0x00]);
    assert_eq!(cpu.register_y, 0x80);
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
}

#[test]
fn test_tya_basic() {
    let cpu = run_program(vec![0xa0, 0x42, 0x98, 0x00]);
    assert_eq!(cpu.register_a, 0x42, "TYA should transfer Y to A");
    assert_flag(&cpu, ZERO, false, "Not zero");
}

#[test]
fn test_tsx_basic() {
    let cpu = run_program_with_setup(vec![0xba, 0x00], |cpu| {
        cpu.stack_pointer = 0x42;
    });
    assert_eq!(cpu.register_x, 0x42, "TSX should transfer SP to X");
    assert_flag(&cpu, ZERO, false, "Not zero");
}

#[test]
fn test_tsx_zero() {
    let cpu = run_program_with_setup(vec![0xba, 0x00], |cpu| {
        cpu.stack_pointer = 0x00;
    });
    assert_eq!(cpu.register_x, 0x00);
    assert_flag(&cpu, ZERO, true, "Zero result");
}

#[test]
fn test_tsx_negative() {
    let cpu = run_program_with_setup(vec![0xba, 0x00], |cpu| {
        cpu.stack_pointer = 0x80;
    });
    assert_eq!(cpu.register_x, 0x80);
    assert_flag(&cpu, NEGETIVE, true, "0x80 sets NEGATIVE");
}

#[test]
fn test_txs_basic() {
    let cpu = run_program_with_setup(vec![0x9a, 0x00], |cpu| {
        cpu.register_x = 0x42;
    });
    assert_eq!(cpu.stack_pointer, 0x42, "TXS should transfer X to SP");
    // TXS does NOT set flags (unlike most transfer instructions)
}

// ============================================================
// IMPROVED EXISTING TESTS
// ============================================================

// Strengthen LDX to verify it doesn't affect A and Y
#[test]
fn test_ldx_does_not_affect_a_y_improved() {
    let cpu = run_program_with_setup(vec![0xa2, 0x42, 0x00], |cpu| {
        cpu.register_a = 0x55;
        cpu.register_y = 0x66;
    });
    assert_eq!(cpu.register_a, 0x55, "LDX should not modify A");
    assert_eq!(cpu.register_y, 0x66, "LDX should not modify Y");
}

// Strengthen LDY to verify it doesn't affect A and X
#[test]
fn test_ldy_does_not_affect_a_x_improved() {
    let cpu = run_program_with_setup(vec![0xa0, 0x42, 0x00], |cpu| {
        cpu.register_a = 0x55;
        cpu.register_x = 0x66;
    });
    assert_eq!(cpu.register_a, 0x55, "LDY should not modify A");
    assert_eq!(cpu.register_x, 0x66, "LDY should not modify X");
}

// Test that branch instructions don't modify registers
#[test]
fn test_branch_does_not_modify_registers() {
    let cpu = run_program_with_setup(vec![0x10, 0x02, 0x00, 0xa9, 0x42, 0x00], |cpu| {
        cpu.register_a = 0x01; // Positive, BPL taken
        cpu.register_x = 0x33;
        cpu.register_y = 0x44;
    });
    assert_eq!(cpu.register_a, 0x42, "BPL taken: should load 0x42");
    assert_eq!(cpu.register_x, 0x33, "BPL should not modify X");
    assert_eq!(cpu.register_y, 0x44, "BPL should not modify Y");
}

// ============================================================
// REGRESSION TESTS (Updated)
// ============================================================

#[test]
fn test_regression_asl_carry_flag() {
    let cpu = run_with_a(0x0a, 0x00, 0x80);
    assert_flag(
        &cpu,
        CARRY,
        true,
        "ASL 0x80: carry should be set from bit 7",
    );
}

#[test]
fn test_regression_lsr_carry_flag() {
    let cpu = run_with_a(0x4a, 0x00, 0x01);
    assert_flag(
        &cpu,
        CARRY,
        true,
        "LSR 0x01: carry should be set from bit 0",
    );
}

#[test]
fn test_regression_cmp_zero_flag() {
    let cpu = run_program_with_setup(vec![0xc9, 0x42, 0x00], |cpu| {
        cpu.register_a = 0x42;
    });
    assert_flag(&cpu, ZERO, true, "CMP: A == M should set ZERO");
    assert_flag(&cpu, CARRY, true, "CMP: A >= M should set CARRY");
}

#[test]
fn test_regression_bit_overflow_flag() {
    let cpu = run_program_with_setup(vec![0x24, 0x44, 0x00], |cpu| {
        cpu.register_a = 0xFF;
        cpu.memory[0x44] = 0x40;
    });
    assert_flag(
        &cpu,
        OVERFLOW,
        true,
        "BIT: memory bit 6 should set OVERFLOW",
    );
}

#[test]
fn test_regression_inc_wrapping() {
    let cpu = run_program_with_setup(vec![0xe6, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0xFF;
    });
    assert_eq!(cpu.mem_read(0x44), 0x00, "INC: 0xFF + 1 = 0x00");
}

#[test]
fn test_regression_dec_wrapping() {
    let cpu = run_program_with_setup(vec![0xc6, 0x44, 0x00], |cpu| {
        cpu.memory[0x44] = 0x00;
    });
    assert_eq!(cpu.mem_read(0x44), 0xFF, "DEC: 0x00 - 1 = 0xFF");
}

// ============================================================
// MEMORY BOUNDARIES AND WRAPPING
// ============================================================

#[test]
fn test_absolute_address_wrap() {
    let cpu = run_program_with_setup(vec![0xad, 0xfe, 0xff, 0x00], |cpu| {
        cpu.memory[0xFFFE] = 0x55;
    });
    assert_eq!(cpu.register_a, 0x55);
}

// ============================================================
// ============================================================
// EDGE CASES: Stack pointer behavior
// ============================================================

#[test]
fn test_jsr_rts_behavior() {
    let mut cpu = CPU::new();

    // Program layout:
    // 0x8000: JSR $8100
    // 0x8003: LDA #$42
    // 0x8005: BRK
    //
    // 0x8100: RTS

    cpu.load_program(vec![
        0x20, 0x00, 0x81, // JSR $8100
        0xA9, 0x42, // LDA #$42 (should execute AFTER RTS)
        0x00, // BRK
    ]);

    cpu.memory[0x8100] = 0x60; // RTS

    cpu.reset();

    let sp_before = cpu.stack_pointer;

    cpu.interpret();

    // ✅ 1. Stack pointer should be restored after JSR + RTS
    assert_eq!(
        cpu.stack_pointer, sp_before,
        "Stack pointer should return to original after RTS"
    );

    // ✅ 2. Ensure execution resumed correctly after JSR
    assert_eq!(
        cpu.register_a, 0x42,
        "RTS should return to instruction after JSR"
    );
}
