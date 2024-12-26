

#define kernel name
CURRENT = $(shell uname -r)
KERNEL_DIR = /lib/modules/$(CURRENT)/build
PWD = $(shell pwd)
BUILD_DIR = $(shell pwd)/.build
BUILD_DIR_MAKEFILE := $(BUILD_DIR)/Makefile

BUILD_DIR_CLIENT = $(shell pwd)/.build

obj-m := fakerandom.o

fakerandom-objs := fakerandom_main.o fakerandom_or.o fakerandom_read.o fakerandom_write.o


$(BUILD_DIR):
	mkdir -p "$@"

$(BUILD_DIR_MAKEFILE): $(BUILD_DIR)
	touch "$@"

all: default typeclient_rebuild


typeclient:
	gcc typeclient.c -o typeclient
	chmod +x typeclient

.PHONY: typeclient_rebuild
typeclient_rebuild:
	rm -f ./typeclient
	gcc typeclient.c -o typeclient
	chmod +x typeclient
# ./typeclient
 

default: $(BUILD_DIR_MAKEFILE)
	$(MAKE) -C $(KERNEL_DIR) M=$(BUILD_DIR) src=$(PWD) modules

.PHONY: clean
clean: $(BUILD_DIR_MAKEFILE)
	$(MAKE) -C $(KERNEL_DIR) M=$(BUILD_DIR) src=$(PWD) clean


.PHONY: build_and_load_mod
build_and_load_mod: default
	modinfo $(BUILD_DIR)/myalert.ko
	rmmod mod_sum_of_array.ko | true
# rmmod mod_sum_of_array.ko | true
# insmod $(BUILD_DIR)/mod_sum_of_array.ko in_array_len=3 nums_array='"57 63 75"'
# tail /var/log/kern.log


