
#include <linux/module.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("user");


// __init - macros 
static int __init hello_init(void)
{
    printk(KERN_ALERT "hello started\n");
    return 0;
}


static void __exit hello_exit(void)
{
    printk(KERN_ALERT "hello exites\n");
    return;
}

module_init(hello_init);
module_exit(hello_exit);