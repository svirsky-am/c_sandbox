/*  radio open and release */  
#include "radio.h"

// #include "linux/fs.h"

static int  radio_count = 0;


int radio_open(struct inode *inode, struct  file *file) 
{
    if(radio_count > 0) return -EBUSY;
    radio_count++;
    printk(KERN_ALERT " in user\n");
    return 0;
}

int radio_release(struct inode *inode, struct  file *file)
{
    radio_count--;
    printk(KERN_ALERT " released\n");
    return 0;
}
