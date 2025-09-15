format ELF64
section ".data" writable
	fmt db "%d", 10, 0
section ".text" executable
public main
extrn printf
main:
	push 69
	mov rdi, fmt
	pop rsi
	xor eax, eax
	sub rsp, 8
	call printf
	add rsp, 8
	xor eax, eax
	ret

