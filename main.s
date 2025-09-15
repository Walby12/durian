format ELF64
section ".data" writable
	fmt db "%d", 10, 0
section ".text" executable
public main
extrn printf
main:
	push 2
	mov rdi, fmt
	mov rsi, [rsp]
	xor eax, eax
	call printf
	add rsp, 8
	xor eax, eax
	ret

