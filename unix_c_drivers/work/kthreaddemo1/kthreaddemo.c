#include <linux/module.h>
#include <linux/kthread.h>
#include <linux/delay.h>


#define STEPS 10

static struct task_struct *thread1;
volatile int flag;


int thread_func(void *arg)
{
    long num = (long)arg;
    int i;

    for (i=0; i<STEPS; i++)
    {
        printk(KERN_ALERT "%ld - %d\n", num, i);
        msleep(500);
    }
    return 0;
}

int init_module(void)
{
    char thread1_name[] = "thread1";
    thread1 = kthread_run(thread_func, (void*)1, thread1_name);
    if(!thread1)
    {
        printk(KERN_ALERT "thread1 cannot start\n");
        return -ENOSYS;
    }

    printk(KERN_ALERT "threads started\n");

    return 0;
}

void cleanup_module(void)
{  

    printk(KERN_ALERT "threaddemo exited\n");
    // return 0;
}


MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");