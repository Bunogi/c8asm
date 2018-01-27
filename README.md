# c8asm
An assembler for the Chip-8 virtual machine written in rust.

Based on Cowgod's [Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

# Syntax
Syntax is as simple as can be, each instruction is named exactly the same as in the reference document. Commas are optional. 

Numbers can be entered in bases 2, 10 and 16. Simply prepend ```0b``` for base 2, and ```0x``` for hex.
## Labels 
```asm
; Wait for delay timer to reach zero, then continue
wait:
ld v0, dt
se v0, 0
jp wait

call do_something ; Do cool stuff
```

## Preprocessor
Currently, only replacing variables is supported.
```asm
#define x_pos v0
#define y_pos v1

; Initialize position
ld $x_pos, 16
ld $y_pos, 8
```

## Embedding data
To embed data into your executable, use the ```db``` "instruction".

```asm
; a plus sprite
db 0b00011000
db 0b00011000
db 0b01111110
db 0b01111110
db 0b00011000
db 0b00011000
````
