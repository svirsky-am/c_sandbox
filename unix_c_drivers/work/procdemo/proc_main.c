#include "proc.h"
#include <linux/proc_fs.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

static struct proc_ops pops = 
{
    .proc_open = node_open,
    .proc_release = node_release,   
    .proc_read = node_read
};


int init_module(void)
{
    struct proc_dir_entry *node = 0;
    if(!(node = proc_create(DEV_NAME, 0, NULL, &pops))) return -EIO;

    printk(KERN_ALERT DEV_NAME " started\n");

    return 0;
}

void cleanup_module(void)
{
    remove_proc_entry(DEV_NAME, NULL);
    printk(KERN_ALERT DEV_NAME " exited\n");
    return ;
}