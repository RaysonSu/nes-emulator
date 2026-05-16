from itertools import product

# note: state transition diagram from https://mermaid.live/edit#pako:eNqtV11zojAU_SuZPNuOEqnVh91paz_HHWbsjha1DxSiMhXCROjW7fS_bz4uFiSlOl19gdyTc--5OQnwhn0WUNzD8xX74y89nqLf_VmMxO9sOqRegJxEIh7R0RFqIqZuxPUPdA4oGQijiAahl1LkBQGn63UYLxToogBqFWf3CwGrGLjUgfNpP4uiDeKyhDBepzzz05DF6GmT0gby4gDRV-pnImUh-qgozjTFxfSKpv4SvXirbM8pfZgCKpToCeUMJd6C6lok-grQ1fDRHx6mWsi1EfQRvzHGwzgQRQboQWFuUS3IVaC7AuYWIgIRcuqnCjAoA3QgR0qAUwAM6cpLwxdd40QHLqEtwiKq_4jNSy26y6JE4YeAl4NnT-sVy0DsvSlQ1jqqg2il408gWtG2ClfDrrR_55xFiM7nAiJlQd1f-QEaf13HoaHXFkCt6VgtbqCMq1yHnjz_GaWsOrc-_TUBTgKcYkq2So1Meyq52TItwnVKuZGrtBluC9q3udIljeWdXhmxdIImTJULdrfHbT5Y3RQqVNkKd3skdGsS3hlZBwVWQbaPFLFlYFEHFlhfzizaX95XF7VYOhDBSg5IgWgZLpYHMKFkla3F8RmxAFn2iVH7gHzebR0rNAYs4UBJCQtj6YhS4Q50wPluBxzogPP_OqAtv2OKvepTvWMsWevS2nlpppY6NS11iNFrTnt6Fb6W1ZlrqCRr1yRrG5MNC6dy3ieZVLd9COs3tKYXLNlUULJlxXYlnC24FyGfZdIMusVzlUDJ2Z1b0rgzuXzy3Nc_PcTDAUq9z61W4jZ3Lee2TG0rBau-Hx3iaF3iCEocWQeZeNeoDyajwmlWceXIMhllZH1ulJFlNMr4cL1j0Dv-nl73EL1jo96xWZNrMBWcY1qBCwpco6nKUDihXPK5U9V28VQnxFuGHPypT6gyE0hx2zVO_mr3lHgrJ5-vNrOupHYHl57lE6iHJZSLJVIL8CSuxJB6z4NmTeR4zIqhnMCanollhfl7pP5Y3wnJKYB9G8lPiAmpHptmNbiBFzwMcG_urda0gSPKI0_e4zdJNcPCbBGd4Z64DDz-PMOz-F1MSrx4wliEe-LlSEzjLFss85ssCcSXSz_0ZL4tggoD8wuZHPfsNrEVB-694VfcO2rZpHNsd0nXbneI3W5apIE3crzZPCatE7t5SmzrtNM66bw38F-VmBw3W91T8e-cdrvEblpWA4tvppTxX_oDTH2Hvf8D2KkymQ

A = "State::ReadOpcode"
B = "State::Implied"
C = "State::Immediate"
D = "State::OneOperandStart"
E = "State::TwoOperandStart"
F = "State::ReadStart"
G = "State::ReadWriteStart"
G2 = "State::ReadWriteExecute"
G3 = "State::ReadWriteCommit"
H = "State::WriteStart"
I = "State::ZeroPageIndexedX"
J = "State::ZeroPageIndexedY"
L = "State::IndexedIndirectStart"
L2 = "State::IndexedIndirectLowByte"
L3 = "State::IndexedIndirectHighByte"
O = "State::IndirectIndexedStart"
O2 = "State::IndirectIndexedLowByte"
O3 = "State::IndirectIndexedHighByte"
O4 = "State::FixHighByte"
R = "State::JumpStart"
R2 = "State::JumpHighByte"
S = "State::AbslouteStart"
S2 = "State::AbslouteAddressHighByte"
V = "State::AbslouteIndexedXStart"
V2 = "State::AbslouteIndexedXHighByte"
W = "State::AbslouteIndexedYStart"
W2 = "State::AbslouteIndexedYHighByte"
Y = "State::AbslouteIndirectStart"
Y2 = "State::AbslouteIndirectHighByte"
Y3 = "State::AbslouteIndirectLowByteActual"
Y4 = "State::AbslouteIndirectHighByteActual"
Z = "State::Relative"
Z2 = "State::RelativeLowByte"
Z3 = "State::RelativeHighByte"

STATES = [A, B, C, D, E, F, G, G2, G3, H, I, J, L, L2, L3, O, O2, O3, O4, R, R2, S, S2, V, V2, W, W2, Y, Y2, Y3, Y4, Z, Z2, Z3]

INSTRUCTIONS = ["BRK", "ORA (d,x)", "STP", "SLO (d,x)", "NOP d", "ORA d ", "ASL d", "SLO d", "PHP", "ORA #i ", "ASL ", "ANC #i", "NOP a", "ORA a ", "ASL a", "SLO a", "BPL *+d", "ORA (d),y", "STP", "SLO (d),y", "NOP d,x", "ORA d,x ", "ASL d,x", "SLO d,x", "CLC", "ORA a,y", "NOP", "SLO a,y", "NOP a,x", "ORA a,x ", "ASL a,x", "SLO a,x", "JSR a ", "AND (d,x)", "STP", "RLA (d,x)", "BIT d ", "AND d", "ROL d", "RLA d", "PLP ", "AND #i", "ROL ", "ANC #i", "BIT a ", "AND a", "ROL a", "RLA a", "BMI *+d ", "AND (d),y", "STP", "RLA (d),y", "NOP d,x ", "AND d,x", "ROL d,x", "RLA d,x", "SEC ", "AND a,y", "NOP", "RLA a,y", "NOP a,x ", "AND a,x", "ROL a,x", "RLA a,x", "RTI", "EOR (d,x)", "STP", "SRE (d,x)", "NOP d", "EOR d", "LSR d", "SRE d", "PHA", "EOR #i", "LSR ", "ALR #i", "JMP a", "EOR a", "LSR a", "SRE a", "BVC *+d", "EOR (d),y", "STP", "SRE (d),y", "NOP d,x", "EOR d,x", "LSR d,x", "SRE d,x", "CLI", "EOR a,y", "NOP", "SRE a,y", "NOP a,x", "EOR a,x", "LSR a,x", "SRE a,x", "RTS ", "ADC (d,x)", "STP", "RRA (d,x)", "NOP d ", "ADC d", "ROR d", "RRA d", "PLA ", "ADC #i", "ROR ", "ARR #i", "JMP (a) ", "ADC a", "ROR a", "RRA a", "BVS *+d ", "ADC (d),y", "STP", "RRA (d),y", "NOP d,x ", "ADC d,x", "ROR d,x", "RRA d,x", "SEI ", "ADC a,y", "NOP", "RRA a,y", "NOP a,x ", "ADC a,x", "ROR a,x", "RRA a,x", "NOP #i", "STA (d,x)", "NOP #i", "SAX (d,x)", "STY d", "STA d", "STX d", "SAX d ", "DEY", "NOP #i", "TXA", "XAA #i", "STY a", "STA a", "STX a", "SAX a", "BCC *+d", "STA (d),y", "STP ", "AHX (d),y", "STY d,x", "STA d,x", "STX d,y", "SAX d,y", "TYA", "STA a,y", "TXS", "TAS a,y", "SHY a,x", "STA a,x", "SHX a,y ", "AHX a,y", "LDY #i", "LDA (d,x)", "LDX #i", "LAX (d,x)", "LDY d", "LDA d", "LDX d", "LAX d", "TAY", "LDA #i", "TAX", "LAX #i", "LDY a", "LDA a", "LDX a", "LAX a", "BCS *+d", "LDA (d),y", "STP", "LAX (d),y", "LDY d,x", "LDA d,x", "LDX d,y", "LAX d,y", "CLV", "LDA a,y", "TSX", "LAS a,y", "LDY a,x", "LDA a,x", "LDX a,y", "LAX a,y", "CPY #i", "CMP (d,x)", "NOP #i ", "DCP (d,x)", "CPY d", "CMP d ", "DEC d ", "DCP d", "INY", "CMP #i ", "DEX ", "AXS #i", "CPY a", "CMP a ", "DEC a ", "DCP a", "BNE *+d", "CMP (d),y", "STP ", "DCP (d),y", "NOP d,x", "CMP d,x ", "DEC d,x ", "DCP d,x", "CLD", "CMP a,y", "NOP ", "DCP a,y", "NOP a,x", "CMP a,x ", "DEC a,x ", "DCP a,x", "CPX #i", "SBC (d,x)", "NOP #i", "ISC (d,x)", "CPX d", "SBC d", "INC d", "ISC d", "INX", "SBC #i", "NOP", "SBC #i", "CPX a", "SBC a", "INC a", "ISC a", "BEQ *+d", "SBC (d),y", "STP", "ISC (d),y", "NOP d,x", "SBC d,x", "INC d,x", "ISC d,x", "SED", "SBC a,y", "NOP", "ISC a,y", "NOP a,x", "SBC a,x", "INC a,x", "ISC a,x"]

READ_INS = "Lda | Ldx | Ldy | Eor | And | Ora | Adc | Sbc | Cmp | Bit | Lax | Nop | Las".upper().split(" | ")
READ_MODIFY_WRITE_INS = "Asl | Lsr | Rol | Ror | Inc | Dec | Slo | Sre | Rla | Rra | Isc | Dcp".upper().split(" | ")
WRITE_INS = "Sta | Stx | Sty | Sax | Ahx | Shx | Shy | Axs".upper().split("|")

READ: list[int] = []
READ_WRITE: list[int] = []
WRITE: list[int] = []

IMPLIED: list[int] = []
IMMEDIATE: list[int] = []
ZERO_PAGE: list[int] = []
ZERO_PAGE_INDEXED_X: list[int] = []
ZERO_PAGE_INDEXED_Y: list[int] = []
INDEXED_INDIRECT: list[int] = []
INDIRECT_INDEXED: list[int] = []
RELATIVE: list[int] = []
ABSLOUTE: list[int] = []
ABSLOUTE_INDEXED_X: list[int] = []
ABSLOUTE_INDEXED_Y: list[int] = []
ABSLOUTE_INDIRECT: list[int] = []

for opcode, ins in enumerate(INSTRUCTIONS):
    lab, addr, *_ = ins.split() + [""]

    if lab in READ_INS:
        READ.append(opcode)

    if lab in READ_MODIFY_WRITE_INS:
        READ_WRITE.append(opcode)

    if lab in WRITE_INS:
        WRITE.append(opcode)
    
    if addr == "":
        IMPLIED.append(opcode)
    elif addr == "#i":
        IMMEDIATE.append(opcode)
    elif addr == "d":
        ZERO_PAGE.append(opcode)
    elif addr == "d,x":
        ZERO_PAGE_INDEXED_X.append(opcode)
    elif addr == "d,y":
        ZERO_PAGE_INDEXED_Y.append(opcode)
    elif addr == "(d,x)":
        INDEXED_INDIRECT.append(opcode)
    elif addr == "(d),y":
        INDIRECT_INDEXED.append(opcode)
    elif addr == "*+d":
        RELATIVE.append(opcode)
    elif addr == "a":
        ABSLOUTE.append(opcode)
    elif addr == "a,x":
        ABSLOUTE_INDEXED_X.append(opcode)
    elif addr == "a,y":
        ABSLOUTE_INDEXED_Y.append(opcode)
    elif addr == "(a)":
        ABSLOUTE_INDIRECT.append(opcode)
    else:
        print(f"Bad instruction: {ins}")

ZERO_OPCODE = IMPLIED + IMMEDIATE
ONE_OPCODE = ZERO_PAGE + ZERO_PAGE_INDEXED_X + ZERO_PAGE_INDEXED_Y + INDEXED_INDIRECT + INDIRECT_INDEXED + RELATIVE
TWO_OPCODE = ABSLOUTE + ABSLOUTE_INDEXED_X + ABSLOUTE_INDEXED_Y + ABSLOUTE_INDIRECT

# flowchart TD
def table(cur: str, ins: int, alt: bool) -> str | None:
    if cur == A:
        # A[Read Opcode] -- 0 opcode --> B
        if ins in IMPLIED:
            return B
        # A -- immediate addressing --> C
        elif ins in IMMEDIATE:
            return C
        # A -- 1 opcode --> D
        elif ins in ONE_OPCODE:
            return D
        # A -- 2 opcode --> E
        elif ins in TWO_OPCODE:
            return E
    elif cur == B:
        # B[Dummy read instruction byte, and execute instruction] --> A
        return A
    elif cur == C:
        # C[Fetch value, and execute instruction] --> A
        return A
    elif cur == D:
        # D[Fetch address] -- Zero page read --> F
        if ins in ZERO_PAGE and ins in READ:
            return F
        # D -- Zero page read-write --> G
        elif ins in ZERO_PAGE and ins in READ_WRITE:
            return G
        # D -- Zero page write --> H
        elif ins in ZERO_PAGE and ins in WRITE:
            return H
        # D -- Zero page indexed X --> I 
        elif ins in ZERO_PAGE_INDEXED_X:
            return I
        # D -- Zero page indexed Y --> J
        elif ins in ZERO_PAGE_INDEXED_Y:
            return J
        # D -- Indexed indirect --> L
        elif ins in INDEXED_INDIRECT:
            return L
        # D -- Indirect Indexed --> O
        elif ins in INDIRECT_INDEXED:
            return O
        # D -- Relative --> Z
        elif ins in RELATIVE:
            return Z
    elif cur == E:
        # E[Fetch low byte of address] -- Jump --> R
        if ins == 0x4C:
            return R
        # E -- Absloute --> S
        elif ins in ABSLOUTE:
            return S
        # E -- Absloute indexed X --> V
        elif ins in ABSLOUTE_INDEXED_X:
            return V
        # E -- Absloute indexed Y --> W
        elif ins in ABSLOUTE_INDEXED_Y:
            return W
        # E -- Absloute indirect Jump --> Y
        elif ins in ABSLOUTE_INDIRECT:
            return Y
    elif cur == F:
        # F[Read from effective address, and execute instruction] --> A 
        return A
    elif cur == G:
        # G[Read from effective address] --> G2
        return G2
    elif cur == G2:
        # G2[Write dummy value back to effective address and execute instruction] --> G3
        return G3
    elif cur == G3:
        # G3[Write result to effective address, and execute instruction] --> A 
        return A
    elif cur == H:
        # H[Write register to effective address] --> A
        return A
    elif cur == I:
        # I[Read from address, then add index X to it] -- read --> F
        if ins in READ:
            return F
        # I -- read-write --> G
        elif ins in READ_WRITE:
            return G
        # I -- write --> H
        elif ins in WRITE:
            return H
    elif cur == J:
        # J[Read from address, then add index Y to it] -- read --> F
        if ins in READ:
            return F
        # J -- write --> H
        elif ins in WRITE:
            return H
    elif cur == L:
        # L[Read from the address, then add index X to it] --> L2
        return L2
    elif cur == L2:
        # L2[Fetch the low byte of the effective address from address] --> L3
        return L3
    elif cur == L3:
        # L3[Fetch the high byte of the effective address from address plus 1 mod 256] -- read --> F
        if ins in READ:
            return F
        # L3 -- read-write --> G
        elif ins in READ_WRITE:
            return G
        # L3 -- write --> H 
        elif ins in WRITE:
            return H
    elif cur == O:
        # O[Fetch pointer address] --> O2
        return O2
    elif cur == O2:
        # O2[Fetch the low byte of the effective address from address] --> O3
        return O3
    elif cur == O3:
        # O3[Fetch the high byte of the effective address from address plus 1 mod 256, and add index Y to the low byte of the effective address] -- oops --> O4
        if alt:
            return O4
        # O3 -- read --> F
        elif ins in READ:
            return F
        # O3 -- read-write --> G
        elif ins in READ_WRITE:
            return G
        # O3 -- write --> H
        elif ins in WRITE:
            return H
    elif cur == O4:
        # O4[Fix high byte of effective address] -- read --> F
        if ins in READ:
            return F
        # O4 -- read-write --> G
        elif ins in READ_WRITE:
            return G
        # O4 -- write --> H
        elif ins in WRITE:
            return H
    elif cur == R:
        # R[Fetch low address byte] --> R2
        return R2
    elif cur == R2:
        # R2[Copy low address byte to low byte of program counter, and fetch high address byte to high byte of program counter] --> A 
        return A
    elif cur == S:
        # S[Fetch low byte of address] --> S2
        return S2
    elif cur == S2:
        # S2[Fetch high byte of address] -- read --> F 
        if ins in READ:
            return F
        # S2 -- read-write --> G 
        elif ins in READ_WRITE:
            return G
        # S2 -- write --> H 
        elif ins in WRITE:
            return H
    elif cur == V:
        # V[Fetch the low byte of the effective address] --> V2
        return V2
    elif cur == V2:
        # V2[Fetch the high byte of the effective address, and add index X to the low byte of it] -- oops --> O4
        if alt:
            return O4
        # V2 -- read --> F
        elif ins in READ:
            return F
        # V2 -- read-write --> G
        elif ins in READ_WRITE:
            return G
        # V2 -- write --> H
        elif ins in WRITE:
            return H
    elif cur == W:
        # W[Fetch the low byte of the effective address] --> W2
        return W2
    elif cur == W2:
        # W2[Fetch the high byte of the effective address, and add index Y to the low byte of it] -- oops --> O4
        if alt:
            return O4
        # W2 -- read --> F
        elif ins in READ:
            return F
        # W2 -- write --> H
        elif ins in WRITE:
            return H
    elif cur == Y:
        # Y[Fetch low byte of pointer] --> Y2
        return Y2
    elif cur == Y2:
        # Y2[Fetch high byte of pointer] --> Y3
        return Y3
    elif cur == Y3:
        # Y3[Fetch low byte of address to latch tmp addr? from pointer] --> Y4
        return Y4
    elif cur == Y4:
        # Y4[Fetch high byte of address to high byte of program counter from pointer plus 1 mod 256, copy latch to low byte of program counter] --> A
        return A
    elif cur == Z:
        # Z[Fetch operand] -- branch --> Z2
        if alt:
            return Z2
        # Z -- no branch --> A
        else:
            return A
    elif cur == Z2:
        # Z2[Add operand to low byte of program counter] -- oops --> Z3
        if alt:
            return Z3
        # Z2 -- no oops --> A 
        else:
            return A
    elif cur == Z3:
        # Z3[Fix high byte of program counter] --> A
        return A
    
    return None

for state, ins, alt in product(STATES, range(256), (True, False)):
    res = table(state, ins, alt)
    if not res:
        print(f"        ({state}, {ins}, {str(alt).lower()}) => None,")
    else:
        print(f"        ({state}, {ins}, {str(alt).lower()}) => Some({table(state, ins, alt)}),")