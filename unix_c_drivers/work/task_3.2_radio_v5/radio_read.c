#include "radio.h"

static char *msg_array[] =
{
    "This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!This is radio-0 speaking!",
    "This is radio-1 speaking!"
};

ssize_t radio_read(struct file *file, char *buffer, size_t length, loff_t *offset){
    
    char *msg = 0;
    int msg_length = 0;
    int ofs = *offset;
    char *p;
    ssize_t len;
    int minor = iminor(file->f_inode);

    if(minor == 2) return -EACCES;
    if (minor>2) return -EINVAL;
    if(strlen(r_buffer) > 0) msg = r_buffer; // > 0 ???? 
    else msg = msg_array[minor];
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