;Register names can also be lowercase
ld V0, 0x20
ld V1, V0
ld I, sprite
drw V0, V1, 5

sprite:
data 0xF0
data 0x90
data 0x90
data 0x90
data 0xF0
