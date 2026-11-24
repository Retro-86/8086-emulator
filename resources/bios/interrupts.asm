timer_interrupt_request:
	push ax

	mov al, 0x20
	out PIC_COMMAND, al

	pop ax
	iret

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