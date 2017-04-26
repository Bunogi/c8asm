ld vd, 0x10
ld ve, 8
ld I, sprite
ld v5, 5

start:
ld dt, v5

wait:
ld v0, dt
se v0, 0
jp wait

draw:
drw vd, ve, 5
jp start

sprite:
db 0xF090
db 0x9090
db 0xF000
