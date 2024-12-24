#include "hello.h"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

int init_module(void)
{
    myalert(DEV_NAME, " started");
    return 0;
}