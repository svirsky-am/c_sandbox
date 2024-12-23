#define kernel name
CURRENT = $(shell uname -r)
KERNEL_DIR = /lib/modules/$(CURRENT)/build
PWD = $(shell pwd)
BUILD_DIR = $(shell pwd)/.build
BUILD_DIR_MAKEFILE := $(BUILD_DIR)/Makefile


obj-m := paramdemo.o

$(BUILD_DIR):
	mkdir -p "$@"

$(BUILD_DIR_MAKEFILE): $(BUILD_DIR)
	touch "$@"

default: $(BUILD_DIR_MAKEFILE)
	$(MAKE) -C $(KERNEL_DIR) M=$(BUILD_DIR) src=$(PWD) modules

.PHONY: clean
clean: $(BUILD_DIR_MAKEFILE)
	$(MAKE) -C $(KERNEL_DIR) M=$(BUILD_DIR) src=$(PWD) clean


.PHONY: build_and_load_mod
build_and_load_mod: default
	modinfo $(BUILD_DIR)/paramdemo.ko
	rmmod paramdemo.ko | true
	insmod $(BUILD_DIR)/paramdemo.ko min_of_range=31 max_of_range=39
	tail /var/log/kern.log


