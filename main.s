format ELF64 executable
entry start
start:
	push 43
	pop rbx
	mov rax, 60
	xor rdi, rdi
	syscall

