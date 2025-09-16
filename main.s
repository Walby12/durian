format ELF64
section ".data" writable
	fmt db "%d", 10, 0
section ".text" executable
public main
extrn printf
extrn putchar
main:
lp:
	push 1
	push 1
	pop rax
	pop rbx
	add rax, rbx
	push rax
	mov rdi, fmt
	pop rsi
	xor eax, eax
	call printf
	cmp rsi, 2
	je lp
a:
	mov rax, 1
	ret
	xor eax, eax
	ret

