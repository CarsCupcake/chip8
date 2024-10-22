This project following this guide: https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

# Missing instructions:

| instructions | Explanation |
| -------------- | --------------- |
| FX07, FX15 and FX18 | Timers |

# Other Plans:
- Write tests
- Make a lang and compile it to chip8 instructions.

# Language: as8
A language for my chip8 processor
| Instruction | Assembly |
| ------------|----------|
| 00E0 | CLEAR |
| 00EE | RET |
| 1NNN | JUMP i |
| 2NNN | CALL i |
| 3XNN | SEQ r i |
| 4XNN | SNEQ r i |
| 5XY0 | REQ x y |
| 6XNN | RSET x i |
| 7XNN | RADD x i |
| 8XY0 | SET x y |
| 8XY1 | OR x y |
| 8XY2 | AND x y |
| 8XY3 | XOR x y |
| 8XY4 | ADD x y |
| 8XY5 | SUB x y |
| 8X06 | SHR x |
| 8XY7 | SUBN x y |
| 8X0E | SHL x |
| 9XY0 | RNEQ x y |
| ANNN | REGI i |
| BNNN | OJMP i |
| DXY0 | DRAW x y |
| EX9E | KSKP x |
| EXA1 | KNSKP x |
| FX07 | REGT x |
| FX0A | KWAIT x |
| FX15 | REGD x |
| FX18 | REGS x |
| FX1E | IADD x |
| FX29 | DSET x |
| FX33 | BCD x |
| FX55 | WRITE x |
| FX65 | READ x |

Maby later: FX75 & FX 85 -> Read write to persistant memory

