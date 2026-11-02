bits 16
cpu 8086

org 0xE000

; PORTS
%define COM1_DATA 0x3F8
%define PIC_COMMAND 0x20
%define PIC_DATA 0x21
%define PIT_CHANNEL_0 0x40
%define PIT_CONTROL 0x43

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

	 mov al, 0xFE
	 out PIC_DATA, al

	 sti

main_loop:
	hlt
	jmp main_loop

timer_interrupt_request:
	push ax
	push dx

	mov dx, COM1_DATA
	mov al, '.'
	out dx, al

	mov al, 0x20
	out PIC_COMMAND, al

	pop dx
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

times 8176 - ($ - start) db 0

reset_vector:
	jmp 0xF000:start

times 8192 - ($ - start) db 0

