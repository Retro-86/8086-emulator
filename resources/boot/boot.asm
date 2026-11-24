bits 16
cpu 8086
org 0x7C00

start:
    push ds
    xor ax, ax
    mov ds, ax
    cld

    mov si, message
    mov dx, 0x3F8

.print:
    lodsb
    test al, al
    jz .stop
    out dx, al
    jmp .print

.stop:
    pop ds
    retf

.halt:
    hlt
    jmp .halt

message: db "Hello World from boot image!", 13, 10, 0

times 510 - ($ - $$) db 0
dw 0xAA55