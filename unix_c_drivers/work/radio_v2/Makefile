

#define kernel name
CURRENT = $(shell uname -r)
KERNEL_DIR = /lib/modules/$(CURRENT)/build
PWD = $(shell pwd)
BUILD_DIR = $(shell pwd)/.build
BUILD_DIR_MAKEFILE := $(BUILD_DIR)/Makefile

obj-m := radio.o

radio-objs := radio_main.o radio_or.o


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
	modinfo $(BUILD_DIR)/myalert.ko
# rmmod mod_sum_of_array.ko | true
# insmod $(BUILD_DIR)/mod_sum_of_array.ko in_array_len=3 nums_array='"57 63 75"'
# tail /var/log/kern.log


