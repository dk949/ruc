%define   	SYS_write       1
%define   	SYS_exit        60

%define     stdout          1

global    _start

section   .text

_start:
          mov       rax, SYS_write
          mov       rdi, stdout
          mov       rsi, message
          mov       rdx, message_len
          syscall

          mov       rax, SYS_exit
          xor       rdi, rdi
          syscall


section   .data
message:    db  "ruc test passed", 10
message_len equ $ -message

section .bss
