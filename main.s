format ELF64
section ".data" writable
	fmt db "%c", 10, 0
section ".text" executable
public main
extrn printf
main:
	push 2
	push 4
	pop rax
	pop rbx
	add rax, rbx
	push rax
	mov rdi, fmt
	pop rax
	add rax, '0'
	mov rsi, rax
	xor eax, eax
	call printf
	xor eax, eax
	ret

