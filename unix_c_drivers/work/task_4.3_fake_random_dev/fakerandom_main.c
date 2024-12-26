#include "fakerandom.h"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

static struct file_operations fops;
static int fakerandom_major;


char r_buffer[BUF_SIZE];

int init_module(void)
{
    memset(&fops, 0, sizeof(fops));
    fops.owner = THIS_MODULE;
    fops.open = fakerandom_open;
    fops.release = fakerandom_release;
    fops.read = fakerandom_read;
    fops.write = fakerandom_write;
    

    fakerandom_major = register_chrdev(0, DEV_NAME, &fops);
    if(fakerandom_major < 0)
    {
        printk(KERN_ALERT DEV_NAME " failed to register\n");
        return fakerandom_major;
    }

    printk(KERN_ALERT DEV_NAME " registered , major number = %d\n", fakerandom_major);

    return 0;
}

void cleanup_module(void)
{
    unregister_chrdev(fakerandom_major, DEV_NAME);
    printk(KERN_ALERT DEV_NAME " exited\n");
    return ;
}