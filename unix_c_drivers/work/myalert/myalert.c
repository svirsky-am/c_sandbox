#include <linux/module.h>

// #define MOD_NAME "myalert"


MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

int myalert(const char *name, const char *msg)
{
    printk(KERN_ALERT "module '%s': %s\n", name , msg);
    return 0;
}

EXPORT_SYMBOL(myalert);