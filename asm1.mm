mov r1, 5
call loop
call fibo
halt

loop:
  loop_2:
  print r1  
  sub r1, 1
  cmp r1,0
  jmpg loop_2
ret

fibo:
mov r0, 10
mov r1, 0
mov r2, 1
mov r3, 0
loop_3:
print r1
mov r3, r1
add r3, r2
mov r1, r2
mov r2, r3
sub r0, 1
cmp r0, 0
jmpg loop_3
ret

