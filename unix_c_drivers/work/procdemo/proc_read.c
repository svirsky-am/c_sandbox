#include "proc.h"


ssize_t node_read(struct file *file, char *buffer, size_t length, loff_t *offset)
{
    char *response = "procdemo response";
    int i, len = strlen(response) + 1;
    loff_t ofs = *offset;

    if(ofs >= len) return 0;

    if(ofs+length > len) length = len-ofs;

    for(i = ofs; i < len; i++) buffer[i] = (response+ofs)[i];

    *offset +=length;
    return length;
}