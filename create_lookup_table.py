# note: state transition diagram from https://mermaid.live/edit#pako:eNqtV11zojAU_SuZPGtHAlrlYXesVmvHHWfsjha1DwhRmQphAnTrdvrfNyTBgkRqp-uTyT333K-TAG_QIS6GJtzsyR9nZ9MY_O6vAsB-3eUU2y6YhCniCdTroAEIX7D_P8CNRKUGz_ex69kxBrbrUhxFXrDloF4OpOW9-zkDyhtuheFm2U98_wBomoIXRDFNnNgjAVgfYlwDduAC_IqdhIXMWZ84RVdQ9JYDHDs78GLvkwtd-tJFVsGLXmBKQGhvscglRQ8kumyu_6FeLAoZKkEf9jul3QtclqQLHjlmBCpBFgfd5zAjaWEIj2In5oBxESAMGTIFTFAOMcV7O_ZeRJILYbiVfWEa4QMAZFPo0X3ihxw_lUy36W53He1JIst9UFqK5c4qMaLa-TmMKOuYiSVxA6HiDSU-wJsNw6S1yeQ_U4Vs_7CKQ0CHMtwQLed8xC6XL9ceWNvOM4hJ2bc6_FCXnLrkZC7JPlYyXVjJ3ZFp60UxpkquwpEY5Wo_xop3OEhXYjZseozGi7kUTg_JKNssHw1uKh2I-wsCWhUB75Ws4xwrI7ukFHZw5FDHSOo_9cyfgXRdHmo-dUkkJznWc0Q7b7v7AhMI90nELlGfuAA1W8rax_r5bgtbrjFSEpPvFjeRxU3-X3FCzSfzvig_3hZCwkikZmSpqbo1qejWRFfKaGIsB95rsTp1DqVgRkUwQxlsipY9Eh540VmfeFDWjHwjQkq21PaBQ5KAnWnRvA0fBU_01LeQ_Ylz8bp4yLRRcFGXeXRR1VkwljU4Q1-Szqk8HlXykNdDSQszpBrPDJ0fzwwpxzP_XtLWV5KeK5OeqxOzlFMLifcxYEueWUs__2TnMrN5geyZmm7-FGe2yCQztIwKqXymugJv6S5w-CEQmVQqv_DgWsh8SIgp6zzv65r9Y1v8zUbe7Yt0PyB5U0aAll02Lel_QeiPsS30jEKyHy3ZyVro5YtEXQ2swS31XGhu7H2Ea9DH1LfTNXxLqVaQacjHK2iyv65Nn1dwFbwzp9AOFoT40GRvAsyNkmS7yxZJ6LKX9b5np_GOCMx0SXtpcGi2tc4154DmG3yFZl1roOurRpPt620NtXW9Bg_Q7Fw19GZbbzU6htHRtE7nvQb_8qjaVRtpGmq2W0anjVqNJqpB9o0QE_pLfHDw7473f4EO2q4

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
O2 = "State::IndirectIndexedLowByte"
O3 = "State::IndirectIndexedHighByte"
O4 = "State::FixHighByte"
R2 = "State::JumpHighByte"
S2 = "State::AbsoluteAddressHighByte"
V2 = "State::AbsoluteIndexedXHighByte"
W2 = "State::AbsoluteIndexedYHighByte"
Y2 = "State::AbsoluteIndirectHighByte"
Y3 = "State::AbsoluteIndirectLowByteActual"
Y4 = "State::AbsoluteIndirectHighByteActual"
Z = "State::Relative"
Z2 = "State::RelativeLowByte"
Z3 = "State::RelativeHighByte"
AA = "State::DummyRead"
AB = "State::DummyReadAndIncrementPC"
AB2 = "State::PushPCHighByte"
AB3 = "State::PushPCLowByte"
AB4 = "State::PushStatusRegisteWithBFlag"
AB5 = "State::FetchPCLowByte"
AB6 = "State::FetchPCHighByte"
AC = "State::IncrementSP"
AC2 = "State::PullStatusRegisterAndIncrementSP"
AC3 = "State::PullPCLowByte"
AC4 = "State::PullPCHighByte"
AD = "State::IncrementPC"
AE = "State::PushAccumulator"
AF = "State::PushStatusRegister"
AG = "State::PullAccumulator"
AH = "State::PullStatusRegister"
AI = "State::FetchSubroutineLowByte"
AI2 = "State::JSRMystery"
AI5 = "State::FetchSubroutineHighByte"

STATES = [A, B, C, D, E, F, G, G2, G3, H, I, J, L, L2, L3, O2, O3, O4, R2, S2, V2, W2, Y2, Y3, Y4, Z, Z2, Z3, AA, AB, AB2, AB3, AB4, AB5, AB6, AC, AC2, AC3, AC4, AD, AE, AF, AG, AH, AI, AI2, AI5]

INSTRUCTIONS = ["BRK", "ORA (d,x)", "STP", "SLO (d,x)", "NOP d", "ORA d ", "ASL d", "SLO d", "PHP", "ORA #i ", "ASL ", "ANC #i", "NOP a", "ORA a ", "ASL a", "SLO a", "BPL *+d", "ORA (d),y", "STP", "SLO (d),y", "NOP d,x", "ORA d,x ", "ASL d,x", "SLO d,x", "CLC", "ORA a,y", "NOP", "SLO a,y", "NOP a,x", "ORA a,x ", "ASL a,x", "SLO a,x", "JSR a ", "AND (d,x)", "STP", "RLA (d,x)", "BIT d ", "AND d", "ROL d", "RLA d", "PLP ", "AND #i", "ROL ", "ANC #i", "BIT a ", "AND a", "ROL a", "RLA a", "BMI *+d ", "AND (d),y", "STP", "RLA (d),y", "NOP d,x ", "AND d,x", "ROL d,x", "RLA d,x", "SEC ", "AND a,y", "NOP", "RLA a,y", "NOP a,x ", "AND a,x", "ROL a,x", "RLA a,x", "RTI", "EOR (d,x)", "STP", "SRE (d,x)", "NOP d", "EOR d", "LSR d", "SRE d", "PHA", "EOR #i", "LSR ", "ALR #i", "JMP a", "EOR a", "LSR a", "SRE a", "BVC *+d", "EOR (d),y", "STP", "SRE (d),y", "NOP d,x", "EOR d,x", "LSR d,x", "SRE d,x", "CLI", "EOR a,y", "NOP", "SRE a,y", "NOP a,x", "EOR a,x", "LSR a,x", "SRE a,x", "RTS ", "ADC (d,x)", "STP", "RRA (d,x)", "NOP d ", "ADC d", "ROR d", "RRA d", "PLA ", "ADC #i", "ROR ", "ARR #i", "JMP (a) ", "ADC a", "ROR a", "RRA a", "BVS *+d ", "ADC (d),y", "STP", "RRA (d),y", "NOP d,x ", "ADC d,x", "ROR d,x", "RRA d,x", "SEI ", "ADC a,y", "NOP", "RRA a,y", "NOP a,x ", "ADC a,x", "ROR a,x", "RRA a,x", "NOP #i", "STA (d,x)", "NOP #i", "SAX (d,x)", "STY d", "STA d", "STX d", "SAX d ", "DEY", "NOP #i", "TXA", "XAA #i", "STY a", "STA a", "STX a", "SAX a", "BCC *+d", "STA (d),y", "STP ", "AHX (d),y", "STY d,x", "STA d,x", "STX d,y", "SAX d,y", "TYA", "STA a,y", "TXS", "TAS a,y", "SHY a,x", "STA a,x", "SHX a,y ", "AHX a,y", "LDY #i", "LDA (d,x)", "LDX #i", "LAX (d,x)", "LDY d", "LDA d", "LDX d", "LAX d", "TAY", "LDA #i", "TAX", "LAX #i", "LDY a", "LDA a", "LDX a", "LAX a", "BCS *+d", "LDA (d),y", "STP", "LAX (d),y", "LDY d,x", "LDA d,x", "LDX d,y", "LAX d,y", "CLV", "LDA a,y", "TSX", "LAS a,y", "LDY a,x", "LDA a,x", "LDX a,y", "LAX a,y", "CPY #i", "CMP (d,x)", "NOP #i ", "DCP (d,x)", "CPY d", "CMP d ", "DEC d ", "DCP d", "INY", "CMP #i ", "DEX ", "AXS #i", "CPY a", "CMP a ", "DEC a ", "DCP a", "BNE *+d", "CMP (d),y", "STP ", "DCP (d),y", "NOP d,x", "CMP d,x ", "DEC d,x ", "DCP d,x", "CLD", "CMP a,y", "NOP ", "DCP a,y", "NOP a,x", "CMP a,x ", "DEC a,x ", "DCP a,x", "CPX #i", "SBC (d,x)", "NOP #i", "ISC (d,x)", "CPX d", "SBC d", "INC d", "ISC d", "INX", "SBC #i", "NOP", "SBC #i", "CPX a", "SBC a", "INC a", "ISC a", "BEQ *+d", "SBC (d),y", "STP", "ISC (d),y", "NOP d,x", "SBC d,x", "INC d,x", "ISC d,x", "SED", "SBC a,y", "NOP", "ISC a,y", "NOP a,x", "SBC a,x", "INC a,x", "ISC a,x"]

READ_INS = "Lda | Ldx | Ldy | Eor | And | Ora | Adc | Sbc | Cmp | Bit | Lax | Nop | Las".upper().split(" | ")
READ_MODIFY_WRITE_INS = "Asl | Lsr | Rol | Ror | Inc | Dec | Slo | Sre | Rla | Rra | Isc | Dcp".upper().split(" | ")
WRITE_INS = "Sta | Stx | Sty | Sax | Ahx | Shx | Shy | Axs".upper().split(" | ")

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
ABSOLUTE: list[int] = []
ABSOLUTE_INDEXED_X: list[int] = []
ABSOLUTE_INDEXED_Y: list[int] = []
ABSOLUTE_INDIRECT: list[int] = []
JMP = 0x4c
JSR = 0x20
RTI = 0x40
RTS = 0x60
PHA = 0x48
PHP = 0x08
PLA = 0x68
PLP = 0x28
BRK = 0x00


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
        ABSOLUTE.append(opcode)
    elif addr == "a,x":
        ABSOLUTE_INDEXED_X.append(opcode)
    elif addr == "a,y":
        ABSOLUTE_INDEXED_Y.append(opcode)
    elif addr == "(a)":
        ABSOLUTE_INDIRECT.append(opcode)
    else:
        print(f"Bad instruction: {ins}")

ZERO_OPCODE = IMPLIED + IMMEDIATE
ONE_OPCODE = ZERO_PAGE + ZERO_PAGE_INDEXED_X + ZERO_PAGE_INDEXED_Y + INDEXED_INDIRECT + INDIRECT_INDEXED + RELATIVE
TWO_OPCODE = ABSOLUTE + ABSOLUTE_INDEXED_X + ABSOLUTE_INDEXED_Y + ABSOLUTE_INDIRECT

# flowchart TD
def table(cur: str, ins: int, alt: bool) -> str | None:
    if cur == A:
        # A[Read Opcode] -- 0 opcode --> B
        if ins in IMPLIED:
            return B
        # A -- immediate addressing --> C
        elif ins in IMMEDIATE:
            return C
        # note this was wrong in the first version
        elif ins in RELATIVE:
            return Z
        # A -- 1 opcode --> D
        elif ins in ONE_OPCODE:
            return D
        # A -- 2 opcode --> E
        elif ins in TWO_OPCODE:
            return E
        # A -- Stack --> AA
        elif ins in [RTI, RTS, PHA, PHP, PLA, PLP]:
            return AA
        # A -- BRK --> AB
        elif ins == BRK:
            return AB
        # A -- JSR --> AI
        elif ins == JSR:
            return AI
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
            return O2
    elif cur == E:
        # E[Fetch low byte of address] -- Jump --> R
        if ins == JMP:
            return R2
        # E -- Absolute --> S
        elif ins in ABSOLUTE:
            return S2
        # E -- Absolute indexed X --> V
        elif ins in ABSOLUTE_INDEXED_X:
            return V2
        # E -- Absolute indexed Y --> W
        elif ins in ABSOLUTE_INDEXED_Y:
            return W2
        # E -- Absolute indirect Jump --> Y
        elif ins in ABSOLUTE_INDIRECT:
            return Y2
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
    elif cur == O2:
        # O2[Fetch the low byte of the effective address from address] --> O3
        return O3
    elif cur == O3:
        # O3[Fetch the high byte of the effective address from address plus 1 mod 256, and add index Y to the low byte of the effective address] -- oops --> O4
        if alt or ins in READ_WRITE or ins in WRITE:
            return O4
        # O3 -- read --> F
        elif ins in READ:
            return F
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
    elif cur == R2:
        # R2[Copy low address byte to low byte of program counter, and fetch high address byte to high byte of program counter] --> A 
        return A
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
    elif cur == V2:
        # V2[Fetch the high byte of the effective address, and add index X to the low byte of it] -- oops --> O4
        if alt or ins in READ_WRITE or ins in WRITE:
            return O4
        # V2 -- read --> F
        elif ins in READ:
            return F
    elif cur == W2:
        # W2[Fetch the high byte of the effective address, and add index Y to the low byte of it] -- oops --> O4
        if alt or ins in WRITE:
            return O4
        elif ins in READ:
            return F
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
    elif cur == AA:
        # AA[Dummy read instruction byte] -- rti,rts,pla,plp --> AC
        if ins in [RTI, RTS, PLA, PLP]:
            return AC
        # AA -- pha --> AE 
        elif ins == PHA:
            return AE
        # AA -- php --> AF
        elif ins == PHP:
            return AF
    elif cur == AB:
        # AB[Dummy read instruction byte, and increment PC] --> AB2
        return AB2
    elif cur == AB2:
        # AB2[Push high byte of program counter onto stack, then decrement S] --> AB3
        return AB3
    elif cur == AB3:
        # AB3[Push low byte of program counter onto stack, decrement S] -- brk --> AB4
        if ins == BRK:
            return AB4
        # AB3 -- jsr --> AI5
        elif ins == JSR:
            return AI5
    elif cur == AB4:
        # AB4[Push status register onto stack, setting B flag, decrement S] --> AB5
        return AB5
    elif cur == AB5:
        # AB5[Fetch low byte of program counter from $FFFE] --> AB6
        return AB6
    elif cur == AB6:
        # AB6[Fetch high byte of program counter from $FFFF] --> A
        return A
    elif cur == AC:
        # AC[Increment S] -- RTI --> AC2
        if ins == RTI:
            return AC2
        # AC -- RTS --> AC3
        elif ins == RTS:
            return AC3
        # AC -- PLA --> AG
        elif ins == PLA:
            return AG
        # AC -- PLP --> AH
        elif ins == PLP:
            return AH
    elif cur == AC2:
        # AC2[Pull status register from stack, then increment S] --> AC3
        return AC3
    elif cur == AC3:
        # AC3[Pull low byte of program counter from stack, then increment S] --> AC4
        return AC4
    elif cur == AC4:
        # AC4[Pull high byte of program counter from stack] -- RTI --> A
        if ins == RTI:
            return A
        # AC4 -- RTS --> AD 
        elif ins == RTS:
            return AD
    elif cur == AD:
        # AD[Increment program counter] --> A
        return A
    elif cur == AE:
        # AE[Push accumulator to stack, then decrement S] --> A
        return A
    elif cur == AF:
        # AF[Push status register onto stack, decrement S] --> A
        return A
    elif cur == AG:
        # AG[Pull accumulator from stack] --> A
        return A
    elif cur == AH:
        # AH[Pull status register from stack] --> A
        return A
    elif cur == AI:
        # AI[Fetches low address to low byte of memory address register, then increment program counter] --> AI2
        return AI2
    elif cur == AI2:
        # AI2[Do nothing] --> AB2
        return AB2
    elif cur == AI5:
        # AI5[Fetch high byte of adress to high byte o program counter, copy low byte of memory address register to low byte of program counter] --> A
        return A
    
    return None

def all_same[T](values: list[T]) -> bool:
    if not values:
        return True
    
    return all(value == values[0] for value in values)

for state in STATES:
    states = [
        (table(state, i, True), table(state, i, False))
        for i in range(256)
    ]

    if all_same(states):
        true, false = states[0]
        if true == false:
            if true:
                print(f"        ({state}, _, _) => Some({true}),")
            else:
                print(f"        ({state}, _, _) => None,")
        else:
            if true:
                print(f"        ({state}, _, true) => Some({true}),")
            else:
                print(f"        ({state}, _, true) => None,")

            if false:
                print(f"        ({state}, _, false) => Some({false}),")
            else:
                print(f"        ({state}, _, false) => None,")
    else:
        for i, (true, false) in enumerate(states):
            if true == false:
                if true:
                    print(f"        ({state}, {i}, _) => Some({true}),")
                else:
                    print(f"        ({state}, {i}, _) => None,")
            else:
                if true:
                    print(f"        ({state}, {i}, true) => Some({true}),")
                else:
                    print(f"        ({state}, {i}, true) => None,")

                if false:
                    print(f"        ({state}, {i}, false) => Some({false}),")
                else:
                    print(f"        ({state}, {i}, false) => None,")

