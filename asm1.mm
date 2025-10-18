mov r0, 2
loop_i:
    mov r3, 1        
    mov r1, 2

loop_j:
    cmp r1, r0
    jmpge end_inner_loop  
    mov r2, r0
    mod r2, r1
    jmpz not_prime
    add r1, 1
    jmp loop_j

not_prime:
    mov r3, 0

end_inner_loop:
    cmp r3, 1
    jmpz do_print     
    jmp skip_print

do_print:
    print r0

skip_print:
    add r0, 1
    cmp r0, 10
    jmple loop_i
halt
