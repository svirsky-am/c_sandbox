#include "hello.h"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("user");

int init_module(void)
{
    printk(KERN_ALERT DEV_NAME " started\n");
    return 0;
}