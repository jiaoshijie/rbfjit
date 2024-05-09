disassemble:
	@cc -o disa hello.s -no-pie -nostdlib -fno-pic
	@objdump -d disa
	@rm -rf disa

test:
	cargo r ./test.bf
