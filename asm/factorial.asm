addi x1, x0, 1
addi x2, x0, 10
addi x3, x0, 0
addi x3, x3, 1
mul x1, x1, x3
bne x3, x2, -8
halt 