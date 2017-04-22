;Flash "0" on the screen repeatedly
start:
LD V0, 0x10
LD V1, 0x10
LD I, 0x0

startloop:
LD V3, 0x2
LD DT, V3 ;flash every two seconds

wait:
LD V3, DT
SE V3, 0 ;if dt == 0 draw
jp wait

draw:
DRW V0, V1, 5
jp startloop
