format ELF64
section ".data" writable
	fmt db "%d", 10, 0
section ".text" executable
public main
extrn printf
extrn putchar
main:
u:
	push 69
	pop rdi
	sub rsp, 8
	call putchar
	add rsp, 8
jmp u
	xor eax, eax
	ret

