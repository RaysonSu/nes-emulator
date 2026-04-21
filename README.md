# Project proposal

## Motivation
This project idea came after watching a [youtube video](https://www.youtube.com/watch?v=oYjYmSniQyM&pp=ygUZaG93IGFjY3VyYXRlIGFyZSBuaW5lbmRpbw%3D%3D) where the creator wrote >100 tests on the accuracy of diffrent NES emulators, and I thought it would be intresting to try and write an emulator that passes as many of these tests as possible. As a bonus I get to play some classic arcade games on my computer.

## Rationale
The NES was chosen as the initial release was more than 40 years ago, and during these 40 years computing hardware has improved to allow for even large performance peanlties due to emulation still allows for real time gameplay of the NES games. Furthermore, the NES uses a Ricoh 2A03 (Although the PAL version uses a Ricoh 2A07. However this won't be important as only the NTSC version will be the main focus of this project.) CPU, which uses the 6502 instruction architecture. The 6502 architecture also only has 6 registers (3 special and 3 general use), and 256 possible opcodes, which means it's relatively simple when compared to modern architectures (x86-64, arm, or even MIPS).

## Project Objectives
The project aims to create an emulator that can
1. Simulate diffrent NES games (e.g. Super Mario Bros).
2. Allow for real time emulation.
3. Be as accurate as possible (meaning it will pass as many of said [tests](https://github.com/100thCoin/AccuracyCoin?tab=readme-ov-file) as possible)

The project also aims to improve my knowledge of 
1. Hardware design
2. NES
3. Doing large projects

## Project Plan
I plan on writing the program in rust for a) increased performance b) improve my knowledge of rust. To keep the code organised the project will use the OOP paradigm, and sepeate code into diffrent modules. The project will also use a test driven development cycle, as there a already a large quantity of tests written already.

## Technical Details
The NES consists of three main chips: the CPU (Ricoh 2A03), the PPU (Ricoh 2C02), and the APU. 

Each of these chips have their own RAM (where the CPU has 2KiB, and the PPU has 16KiB). However, the CPU (and the PPU) both can address more than the size of their RAM to perform different functions. This addressing is done with a memory mapping chip, where the 64KiB address space is split into diffrent areas to access diffrent parts of the NES.

For the CPU the area is split into:

- $0000 - $00FF: The zero page (in RAM) - this area allows for memory to be addressed with a single byte instead of two, which means read/write operations take 2 instead of three clock cycles.
- $0100 - $01FF: The stack - this area allows for return addresses to be stored during calls to subroutines. The stack (usually) starts from $01FF and grows down to $0100, and is indexed into by the stack pointer.
- $0200 - $07FF: More RAM - acts the same as the zero page, except it requires more 2 bytes to address this space instead of 1.
- $0800 - $1FFF: Mirrors of $0000 - $07FF - writes/read to this area (e.g. $1234) have the same behavior as writing to the address mod 0x800 (so writing to $1234 % 0x800 = writing to $0234).
- $2000 - $2007: PPU control - these addresses do some black magic (TODO: read more into this)
- $2008 - $2FFF: Mirrors of $2000 - $2007 - similar to $0800 - $1FFF
- $4000 - $4013: APU control - when written to, these addresses interact with the APU (TODO: read more into this). However, when read from, these addresses act as open bus
- $4014 - $4017: Does something - TODO: wtf is this
- $4018 - $401F: Does nothing - this area had some scrapped test functionality, writes to this area do nothing? (TODO: check this), and reads to this area return random values (TODO: also check this).
- $4020 - $FFFF: Does something - This area is default unmapped, but is probably used to interact with the game cartridge.

For the PPU the area is split into:

??????????????????

The CPU, and PPU are synchronised to a master clock (which runs at 236.25 / 11 MHz). The CPU clock runs at a speed 12x slower than the master clock, and PPU clock runs at a speed 4x slower than the master clock. However, the PPU clock may not be synced to the CPU clock (e.g. the 1st CPU clock cycle could be synced to the 2nd PPU clock cycle). 

The CPU can also only read or write a single byte of data from the MDR at a time, therefore instructions may take multiple cycles to execute. An example is to execute the intruction INC $01 (E6 01), and say the value 37 is stored at memory address $0001, requires the CPU to:
1. read E6 on the first cycle, decoding it as the INC instruction with zero page addressing mode, then increment the program counter.
2. read 01 on the second cycle, completeing the decoding of the instuction as INC $01, then increment the program counter again.
3. read 37 on the third cycle, getting the value from memory to process
4. compute 37 + 1 = 38 on the fourth cycle, which sets the negative and zero flags (to zero in this case)
5. writes 38 into address $01 on the fifth cycle, completeing the instruction.
