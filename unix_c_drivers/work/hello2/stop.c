#include "hello.h"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("user");

void cleanup_module(void)
{
    printk(KERN_ALERT DEV_NAME " exited\n");
    return;
}