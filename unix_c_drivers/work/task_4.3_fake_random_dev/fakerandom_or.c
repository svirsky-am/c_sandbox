/*  fakerandom open and release */  
#include "fakerandom.h"

// #include "linux/fs.h"

static int  fakerandom_count = 0;


int fakerandom_open(struct inode *inode, struct  file *file) 
{
    if(fakerandom_count > 0) return -EBUSY;
    fakerandom_count++;
    printk(KERN_ALERT " in user\n");
    return 0;
}

int fakerandom_release(struct inode *inode, struct  file *file)
{
    fakerandom_count--;
    printk(KERN_ALERT " released\n");
    return 0;
}
