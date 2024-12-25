#include "proc.h"
#include <linux/sched.h>


DECLARE_WAIT_QUEUE_HEAD(WaitQ);

static int already_opened = 0;

int node_open(struct inode *inode, struct file *file)
{
    int i, is_sig = 0;
    if(already_opened) if (file -> f_flags & O_NONBLOCK) return -EAGAIN;

    while(already_opened)
    {
        wait_event_interruptible(WaitQ, !already_opened);
        for(i = 0; i < _NSIG_WORDS && !is_sig; i++)
         {
            is_sig = current -> pending.signal.sig[i] & ~current->blocked.sig[i]; // & ~ check if not blocked            
        }
        if(is_sig) return -EINTR;
    }   
    
    already_opened = 1;
    return 0;
}

int node_release(struct inode *inode, struct file *file)
{
    already_opened = 0;
    wake_up(&WaitQ);
    return 0;

}