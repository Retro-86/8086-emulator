%define FLOPPY_OUTPUT 0x3F2
%define FLOPPY_STATUS 0x3F4
%define FLOPPY_DATA 0x3F5

%define BOOT_ADDRESS 0x7C00

boot:
    mov dx, FLOPPY_OUTPUT
    mov al, 0x14
    out dx, al

    mov dx, FLOPPY_DATA
    mov si, boot_commands
    mov cx, boot_commands_end - boot_commands
    cld

.send_command:
    lodsb
    out dx, al
    loop .send_command

    xor ax, ax
    mov es, ax
    mov di, BOOT_ADDRESS
    mov cx, 512

.read_sector:
    mov dx, FLOPPY_STATUS
    in al, dx
    cmp al, 0xF0
    jne .failed

    mov dx, FLOPPY_DATA
    in al, dx
    mov [es:di], al
    inc di
    loop .read_sector

    mov dx, FLOPPY_STATUS
    in al, dx
    cmp al, 0xD0
    jne .failed

    mov dx, FLOPPY_DATA
    in al, dx
    and al, 0xC0
    mov bl, al
    in al, dx
    or bl, al
    in al, dx
    or bl, al

    mov cx, 4

.read_remaining_result:
    in al, dx
    loop .read_remaining_result

    test bl, bl
    jnz .failed
    cmp word [es:BOOT_ADDRESS + 510], 0xAA55
    jne .failed

    xor dl, dl
    call 0x0000:BOOT_ADDRESS
    ret

.failed:
    mov si, boot_error
    call print_string
    ret

boot_commands:
    db 0x03, 0xDF, 0x03
    db 0x46, 0, 0, 0, 1, 2, 1, 0xFF, 0
boot_commands_end:

boot_error:
    db "Floppy boot failed", 13, 10, 0