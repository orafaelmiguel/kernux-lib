.PHONY: all clean

all:
	@echo "This Makefile is intended to be used by an external kernel module's build process."
	@echo "To build a driver using this crate, run 'make -C /path/to/kernel/sources M=$(PWD)/examples/hello_driver modules'"

clean:
	@echo "Run clean from within a driver's directory."