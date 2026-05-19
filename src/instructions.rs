use Instruction::*;

pub const INSTRUCTION_TABLE: [Instruction; 256] = [
    Brk, Ora, Stp, Slo, Nop, Ora, Asl, Slo, Php, Ora, Asl, Anc, Nop, Ora, Asl, Slo,
    Bpl, Ora, Stp, Slo, Nop, Ora, Asl, Slo, Clc, Ora, Nop, Slo, Nop, Ora, Asl, Slo,
    Jsr, And, Stp, Rla, Bit, And, Rol, Rla, Plp, And, Rol, Anc, Bit, And, Rol, Rla,
    Bmi, And, Stp, Rla, Nop, And, Rol, Rla, Sec, And, Nop, Rla, Nop, And, Rol, Rla,
    Rti, Eor, Stp, Sre, Nop, Eor, Lsr, Sre, Pha, Eor, Lsr, Alr, Jmp, Eor, Lsr, Sre,
    Bvc, Eor, Stp, Sre, Nop, Eor, Lsr, Sre, Cli, Eor, Nop, Sre, Nop, Eor, Lsr, Sre,
    Rts, Adc, Stp, Rra, Nop, Adc, Ror, Rra, Pla, Adc, Ror, Arr, Jmp, Adc, Ror, Rra,
    Bvs, Adc, Stp, Rra, Nop, Adc, Ror, Rra, Sei, Adc, Nop, Rra, Nop, Adc, Ror, Rra,
    Nop, Sta, Nop, Sax, Sty, Sta, Stx, Sax, Dey, Nop, Txa, Xaa, Sty, Sta, Stx, Sax,
    Bcc, Sta, Stp, Ahx, Sty, Sta, Stx, Sax, Tya, Sta, Txs, Tas, Shy, Sta, Shx, Ahx,
    Ldy, Lda, Ldx, Lax, Ldy, Lda, Ldx, Lax, Tay, Lda, Tax, Lax, Ldy, Lda, Ldx, Lax,
    Bcs, Lda, Stp, Lax, Ldy, Lda, Ldx, Lax, Clv, Lda, Tsx, Las, Ldy, Lda, Ldx, Lax,
    Cpy, Cmp, Nop, Dcp, Cpy, Cmp, Dec, Dcp, Iny, Cmp, Dex, Axs, Cpy, Cmp, Dec, Dcp,
    Bne, Cmp, Stp, Dcp, Nop, Cmp, Dec, Dcp, Cld, Cmp, Nop, Dcp, Nop, Cmp, Dec, Dcp,
    Cpx, Sbc, Nop, Isc, Cpx, Sbc, Inc, Isc, Inx, Sbc, Nop, Sbc, Cpx, Sbc, Inc, Isc,
    Beq, Sbc, Stp, Isc, Nop, Sbc, Inc, Isc, Sed, Sbc, Nop, Isc, Nop, Sbc, Inc, Isc
];


#[derive(PartialEq, Clone, Debug)]
pub enum Instruction {
    Lda, Sta, Ldx, Stx, Ldy, Sty,
    Tax, Txa, Tay, Tya,
    Adc, Sbc, Inc, Dec, Inx, Dex, Iny, Dey,
    Asl, Lsr, Rol, Ror,
    And, Ora, Eor, Bit,
    Cmp, Cpx, Cpy,
    Bcc, Bcs, Beq, Bne, Bpl, Bmi, Bvc, Bvs,
    Jmp, Jsr, Rts, Brk, Rti,
    Pha, Pla, Php, Plp, Txs, Tsx,
    Clc, Sec, Cli, Sei, Cld, Sed, Clv,
    Nop,

    // unofficial opcodes
    Ahx, Alr, Arr, Axs, Anc, Dcp, Isc, Las, Lax, Rla, Rra, Sax, Shx, Shy, Slo,  Sre, Stp, Tas, Xaa
}

impl Instruction {
    pub fn from_opcode(opcode: u8) -> Instruction {
        return INSTRUCTION_TABLE[opcode as usize].clone();
    }
}