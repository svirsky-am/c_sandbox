

#define kernel name
CURRENT = $(shell uname -r)
KERNEL_DIR = /lib/modules/$(CURRENT)/build
PWD = $(shell pwd)
BUILD_DIR = $(shell pwd)/.build
BUILD_DIR_MAKEFILE := $(BUILD_DIR)/Makefile

obj-m := five_chinese.o


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
	modinfo $(BUILD_DIR)/five_chinese.ko
	rmmod five_chinese.ko | true
	insmod $(BUILD_DIR)/five_chinese.ko
# tail /var/log/kern.log


