CROSS ?= aarch64-linux-gnu

all: hello.bin

hello.elf: hello.S linker.ld
	$(CROSS)-gcc -nostdlib -nostartfiles -T linker.ld -o $@ hello.S

hello.bin: hello.elf
	$(CROSS)-objcopy -O binary $< $@

clean:
	rm -f *.elf *.bin

.PHONY: all clean
