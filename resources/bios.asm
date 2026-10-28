bits 16
cpu 8086

org 0xE000

%define COM1_DATA 0x3F8

start:
    cli
    cld
    
    mov ax, 0xF000
    mov ds, ax
    mov es, ax
    mov ax, 0x0000
    mov ss, ax
    mov sp, 0x7C00

    mov si, boot_msg
    call print_string_emu

main_loop:
    mov al, '.'
    mov dx, COM1_DATA
    out dx, al                  

    mov cx, 0xFFFF
delay:
    loop delay
    jmp main_loop

print_string_emu:
    push ax
    push si
    mov dx, COM1_DATA
.loop:
    lodsb                       
    or al, al                   
    jz .done
    out dx, al                  
    jmp .loop
.done:
    pop si
    pop ax
    ret

boot_msg: db "BIOS starting...", 0x0D, 0x0A, 0

times 8176 - ($ - start) db 0

reset_vector:
    jmp 0xF000:start            

times 8192 - ($ - start) db 0

