addi x1, x0, 1
addi x2, x0, 1
addi x4, x0, 2
addi x5, x0, 7
add x3, x1, x2
add x1, x0, x2
add x2, x0, x3
addi x4, x4, 1
bne x4, x5, -16
halt