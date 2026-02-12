# Project proposal

## Motivation
This project idea came after watching a [youtube video](https://www.youtube.com/watch?v=oYjYmSniQyM&pp=ygUZaG93IGFjY3VyYXRlIGFyZSBuaW5lbmRpbw%3D%3D) where the creator wrote >100 tests on the accuracy of diffrent NES emulators, and I thought it would be intresting to try and write an emulator that passes as many of these tests as possible. As a bonus I get to play some classic arcade games on my computer.

## Rationale

The NES was chosen as the initial release was more than 40 years ago, and during these 40 years computing hardware has improved to allow for even large performance peanlties due to emulation still allows for real time gameplay of the NES games. Furthermore, the NES uses a Ricoh 2A03 CPU, which uses the 6502 instruction architecture. The 6502 architecture also only has 6 registers (3 special and 3 general use), and 256 possible opcodes, which means it's relatively simple when compared to modern architectures (x86-64, arm, or even MIPS).

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

