CROSS ?= aarch64-linux-gnu

all: hello.elf shell.elf

hello.elf: hello.S linker.ld
	$(CROSS)-gcc -nostdlib -nostartfiles -T linker.ld -o $@ hello.S

shell.elf: shell.S linker.ld
	$(CROSS)-gcc -nostdlib -nostartfiles -T linker.ld -o $@ shell.S

hello.bin: hello.elf
	$(CROSS)-objcopy -O binary $< $@

clean:
	rm -f *.elf *.bin

.PHONY: all clean
