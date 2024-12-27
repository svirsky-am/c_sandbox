#include "fakerandom.h"


/*------------------ global variables start----------------*/

static int in_array_len = 5;
static int summator_dev_count = 0;
static int min_of_range = 1;
static int max_of_range = 10;
static struct file_operations fops;
static int summator_dev_major;


ssize_t fakerandom_write(struct file *file, const char *buffer, size_t length, loff_t *offset)
{
    int ofs = *offset;
    char *p;
    ssize_t len;
    int minor = iminor(file->f_inode);

    if(minor <2 ) return -EACCES;
    if (minor>2) return -EINVAL;

    if(ofs >= BUF_SIZE-1) return 0;


    if(ofs+length > BUF_SIZE-1) length = BUF_SIZE-1 - ofs;

    len = length;
    for(p = r_buffer+ofs; len > 0; p++, len--, buffer++) get_user(*p, buffer);
    *offset += length;

    *(p) = 0;

    return length;

}