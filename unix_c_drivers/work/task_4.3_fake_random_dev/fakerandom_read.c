#include "fakerandom.h"

static char *msg_array = "This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!This is fakerandom-0 speaking!";

ssize_t fakerandom_read(struct file *file, char *buffer, size_t length, loff_t *offset){
    
    char *msg = 0;
    int msg_length = 0;
    int ofs = *offset;
    char *p;
    ssize_t len;
    int minor = iminor(file->f_inode);

    // if(minor == 2) return -EACCES;
    if(!minor==1) return -EINVAL;
    if(strlen(r_buffer) > 0) msg = r_buffer; // > 0 ???? 
    else msg = msg_array;
    msg_length = strlen(msg);

    if(ofs >= msg_length)
    {
        *r_buffer = 0;
        return 0;
    }

    if (ofs >= msg_length) return 0;
    if(ofs+length > msg_length) length = msg_length - ofs;

    len = length;
    for(p = msg+ofs; len > 0; p++, len--, buffer++) put_user(*p, buffer);
    *offset += length;

    return length;

}