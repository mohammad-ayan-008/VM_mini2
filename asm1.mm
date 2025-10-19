.data
db num 10
db num2 -20
db num_prime 10

.code
mov r7,num2
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
    cmp r0, num_prime
    jmple loop_i
halt

;mov r1, 5
;call loop
;call fibo
;halt

; loop:
;   loop_2:
;   print r1  
;   sub r1, 1
;   cmp r1,0
;   jmpg loop_2
; ret
;
; fibo:
; mov r0, 10
; mov r1, 0
; mov r2, 1
; mov r3, 0
; loop_3:
; print r1
; mov r3, r1
; add r3, r2
; mov r1, r2
; mov r2, r3
; sub r0, 1
; cmp r0, 0
; jmpg loop_3
; ret

