#ifndef _fakerandom_H_
#define _fakerandom_H_

#include <linux/module.h>

#define DEV_NAME "fakerandom"

#ifndef BUF_SIZE
#define BUF_SIZE 64
#endif /*BUF_SIZE*/

extern char r_buffer[BUF_SIZE];


int fakerandom_open(struct inode*, struct file*);
int fakerandom_release(struct inode*, struct file*);

ssize_t fakerandom_read(struct file*, char*, size_t, loff_t*);
ssize_t fakerandom_write(struct file*, const char*, size_t, loff_t*);

#endif /*_fakerandom_H_*/