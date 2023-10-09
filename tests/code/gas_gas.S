.set   	SYS_write, 1
.set   	SYS_exit,  60

.set    stdout,    1

.global _start

.text
_start:
        mov     $SYS_write, %rax
        mov     $stdout, %rdi
        mov     $message, %rsi
        mov     $message_len, %rdx
        syscall

        mov     $SYS_exit, %rax
        xor     %rdi, %rdi
        syscall
.data
message: .ascii  "ruc test passed\n"
message_len = . - message

