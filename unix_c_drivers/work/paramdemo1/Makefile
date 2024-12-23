#define kernel name
CURRENT = $(shell uname -r)
KERNEL_DIR = /lib/modules/$(CURRENT)/build
PWD = $(shell pwd)
BUILD_DIR = $(shell pwd)/.build

obj-m := paramdemo.o

.PHONY: prepare_build_dir
prepare_build_dir:
	rm -rf $(BUILD_DIR)
	mkdir -p $(BUILD_DIR)	
# cp *.c *.h $(BUILD_DIR)


default: prepare_build_dir
	cd $(BUILD_DIR) && $(MAKE) -C $(KERNEL_DIR) M=$(PWD) modules
# default:
#	$(MAKE) -C $(KERNEL_DIR) M=$(PWD) modules
