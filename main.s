format ELF64
section ".data" writable
	fmt db "%d", 10, 0
section ".text" executable
public main
extrn printf
main:
	push 8
	push 2
	mov rax, [rsp]
	push rax
	push 2
	mov rdi, fmt
	pop rsi
	xor eax, eax
	call printf
	xor eax, eax
	ret

