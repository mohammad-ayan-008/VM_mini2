mov r1, 5
loop:
print r1
sub r1, 1
cmp r1, 0
jmpg loop
exit:
halt
