#include "hello.h"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("user");

void cleanup_module(void)
{
    myalert( DEV_NAME, " exited\n");
    return;
}