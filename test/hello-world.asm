; not written by me

section .data
    msg db "Hello, World!", 0xA  ; Message with newline
    len equ $ - msg              ; Length of the message

section .text
    global _start

_start:
    ; Write the message to stdout
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; file descriptor: stdout
    mov rsi, msg        ; address of the message
    mov rdx, len        ; length of the message
    syscall

    ; Exit the program
    mov rax, 60         ; syscall: exit
    xor rdi, rdi        ; status: 0
    syscall
