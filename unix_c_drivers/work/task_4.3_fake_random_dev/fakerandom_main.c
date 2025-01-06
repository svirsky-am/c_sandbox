#include "fakerandom.h"
#include <linux/kthread.h>
#include <linux/delay.h>


MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

#define STEPS 100
#define MOD_NAME "fakerandom"


/*------------------global vars----------------*/

static struct task_struct *thread1;
static struct task_struct *thread2;
volatile int flag;
static struct file_operations fops;
static int fakerandom_major;

static int in_array_len = 5;

static int min_of_range = 1;
static int max_of_range = 10;


char r_buffer[BUF_SIZE];



/*-----------------------lib func----------------*/



int generate_simple_random(int *lower, int *upper)
{
    unsigned int some_random_i;
    get_random_bytes(&some_random_i, sizeof(some_random_i)-1);  
    int mod_lower = some_random_i % *lower;
    int num = (some_random_i % (*upper - *lower + 1)) + *lower;
    printk(KERN_ALERT MOD_NAME "\tmod_lower = %d, random number = %d \tresult = %d", mod_lower, some_random_i, num);
    

    return num;
}


/*value set func for callback*/

static int value_set(const char *val, const struct kernel_param *kp )
{   
    int old = max_of_range;
    int ret, res;

    ret = kstrtoint(val, 10, &res);
    if(ret != 0 || res < 0 || res > 100) return -EINVAL;

    param_set_int(val, kp);

    printk(KERN_ALERT MOD_NAME " old value = %d, new value = %d\n", old, max_of_range);
    int mun = generate_simple_random(&min_of_range, &max_of_range);
    printk(KERN_ALERT MOD_NAME "reseted random number = %d ", mun);
    return 0;
};

/*---------------------- callback structure--------------------------------*/

static const struct kernel_param_ops kpops = {
    .set = value_set, 
    .get = param_get_int
};

/*----------------- declare module patams ------------------- */



module_param(in_array_len, int, 0664);
MODULE_PARM_DESC(in_array_len, "Len of input array: in_array_len");

module_param_cb(min_of_range, &kpops, &min_of_range, 0664);
MODULE_PARM_DESC(min_of_range, "module counter min_of_range");

module_param_cb(max_of_range, &kpops, &max_of_range, 0664);
MODULE_PARM_DESC(max_of_range, "module counter max_of_range");


/*----------------------worker functions --------------------------------------*/


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



int thread_func_tranciver(void *arg)
{
    
    memset(&fops, 0, sizeof(fops));
    fops.owner = THIS_MODULE;
    fops.open = fakerandom_open;
    fops.release = fakerandom_release;
    fops.read = fakerandom_read;
    fops.write = fakerandom_write;
    

    fakerandom_major = register_chrdev(0, DEV_NAME, &fops);
    if(fakerandom_major < 0)
    {
        printk(KERN_ALERT DEV_NAME " failed to register\n");
        return fakerandom_major;
    }

    printk(KERN_ALERT DEV_NAME " registered , major number = %d\n", fakerandom_major);
    return 0;
}

/*---------------------------------------------------------------------------*/

int init_module(void)
{
    char thread1_name[] = "thread1";
    char thread2_name[] = "thread2";

    thread1 = kthread_create(thread_func_tranciver, (void*)1, thread1_name);
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

    unregister_chrdev(fakerandom_major, DEV_NAME);
    printk(KERN_ALERT DEV_NAME " exited\n");
    return ;
}