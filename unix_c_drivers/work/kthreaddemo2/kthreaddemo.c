#include <linux/module.h>
#include <linux/kthread.h>
#include <linux/delay.h>


#define STEPS 100

static struct task_struct *thread1;
static struct task_struct *thread2;
volatile int flag;


int thread_func(void *arg)
{
    long num = (long)arg;
    int i;

    for (i=0; i<STEPS; i++)
    {
        if(i%10==0) printk(KERN_ALERT "%ld - %d\n", num, i);

        msleep(1000);

        if(kthread_should_stop()) break;
    }
    return 0;
}

int init_module(void)
{
    char thread1_name[] = "thread1";
    char thread2_name[] = "thread2";

    thread1 = kthread_create(thread_func, (void*)1, thread1_name);
    if(!thread1)
    {
        printk(KERN_ALERT "thread1 cannot start\n");
        return -ENOSYS;
    }
    get_task_struct(thread1);
    wake_up_process(thread1);

//just crate thread    
    thread2 = kthread_create(thread_func, (void*)1, thread2_name);

    if(!thread2)
    {
        printk(KERN_ALERT "thread1 cannot start\n");
        return -ENOSYS;
    }
//run thread
    get_task_struct(thread2);
    wake_up_process(thread2);


    printk(KERN_ALERT "threads started\n");

    return 0;
}

void cleanup_module(void)
{  
    kthread_stop(thread1);
    put_task_struct(thread1);

    kthread_stop(thread2);
    put_task_struct(thread2);
    printk(KERN_ALERT "threaddemo exited\n");
    // return 0;
}


MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");