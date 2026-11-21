bits 16
cpu 8086

org 0xE000

; PORTS
%define COM1_DATA 0x3F8
%define PIC_COMMAND 0x20
%define PIC_DATA 0x21
%define PIT_CHANNEL_0 0x40
%define PIT_CONTROL 0x43
%define KEYBOARD_PORT 0x60

start:
	cli
	cld

	mov ax, 0xF000
	mov ds, ax
	mov es, ax

	mov ax, 0x0000
	mov ss, ax
	mov sp, 0x7C00

	mov si, boot_message
	call print_string

	push ds
	mov ax, 0x0000
	mov ds, ax

	mov ax, timer_interrupt_request
	mov [0x0020], ax

	mov ax, 0xF000
	mov [0x0022], ax

	mov ax, keyboard_interrupt_request
	mov [0x0024], ax

	mov ax, 0xF000
	mov [0x0026], ax

	pop ds

	mov al, 0x34
	out PIT_CONTROL, al

	mov al, 0
	out PIT_CHANNEL_0, al
	out PIT_CHANNEL_0, al

	mov al, 0x13
	out PIC_COMMAND, al

	mov al, 0x08
	out PIC_DATA, al

	mov al, 0x01
	out PIC_DATA, al

	 mov al, 0xFC
	 out PIC_DATA, al

	 sti

main_loop:
	hlt
	jmp main_loop

timer_interrupt_request:
	push ax

	mov al, 0x20
	out PIC_COMMAND, al

	pop ax
	iret

print_string:
	push ax
	push dx
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
	pop dx
	pop ax
	ret

boot_message: db "BIOS starting...", 0x0D, 0x0A, 0

keyboard_interrupt_request:
    push ax
    push bx
    push dx
    push ds

    push cs
    pop ds

    in al, KEYBOARD_PORT

    test al, 0x80
	jnz .eoi

    cmp al, ascii_table_end - ascii_table
	jae .eoi

	xor ah, ah
	mov bx, ax
	mov al, [ascii_table + bx]

	test al, al
	jz .eoi

	mov dx, COM1_DATA

	cmp al, 0x0D
	jne .print

	out dx, al
	mov al, 0x0A

.print:
	out dx, al

.eoi:
	mov al, 0x20
	out PIC_COMMAND, al

	pop ds
	pop dx
	pop bx
	pop ax
	iret

ascii_table:
	db 0, 0
	db "1234567890-="
	db 8, 9
	db "qwertyuiop[]"
	db 13, 0
	db "asdfghjkl;'"
	db 0x60
	db 0, 0x5C
	db "zxcvbnm,./"
	db 0, 0x2A, 0, 0x20
ascii_table_end:

times 8176 - ($ - start) db 0

reset_vector:
	jmp 0xF000:start

times 8192 - ($ - start) db 0
