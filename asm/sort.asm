lui x7, 0x8000
addi x7, x7, 0x004c
lw x1, -4(x7)
slli x1, x1, 2
addi x2, x0, 4
add x3, x0, x0
sub x8, x1, x2
add x4, x3, x7
lw x5, 0(x4)
lw x6, +4(x4)
blt x5, x6, +12
sw x6, 0(x4)
sw x5, 4(x4)
addi x3, x3, 4
bne x3, x8, -28
addi x2, x2, 4
bne x2, x1, -44
halt 