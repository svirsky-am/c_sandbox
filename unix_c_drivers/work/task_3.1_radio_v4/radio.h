#ifndef _RADIO_H_
#define _RADIO_H_

#include <linux/module.h>

#define DEV_NAME "radio"

#ifndef BUF_SIZE
#define BUF_SIZE 1024
#endif /*BUF_SIZE*/

extern char r_buffer[BUF_SIZE];


int radio_open(struct inode*, struct file*);
int radio_release(struct inode*, struct file*);

ssize_t radio_read(struct file*, char*, size_t, loff_t*);
ssize_t radio_write(struct file*, const char*, size_t, loff_t*);

#endif /*_RADIO_H_*/