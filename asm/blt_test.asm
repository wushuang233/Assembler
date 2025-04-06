# 测试blt指令的循环功能
# 计算1到10的和

addi x1, x0, 0    # x1 = 0 (累加器，存储结果)
addi x2, x0, 10   # x2 = 10 (循环上限)
addi x3, x0, 0    # x3 = 0 (循环计数器)
addi x3, x3, 1    # x3 += 1
add x1, x1, x3    # x1 += x3
blt x3, x2, -8    # 如果x3 < x2, 跳回到loop
halt              # 程序结束 