#include <linux/module.h>
#include <linux/kthread.h>
#include <linux/delay.h>


#define STEPS 100


static int min_of_range = 70;
static int max_of_range = 200;


static struct task_struct *t_chinese_1;
static struct task_struct *t_chinese_2;
static struct task_struct *t_chinese_3;
static struct task_struct *t_chinese_4;
static struct task_struct *t_chinese_5;


volatile int flag;

static struct mutex mtx12;
static struct mutex mtx23;
static struct mutex mtx34;
static struct mutex mtx45;
static struct mutex mtx51;


int generate_simple_random(int *lower, int *upper)
{
    unsigned int some_random_i;
    get_random_bytes(&some_random_i, sizeof(some_random_i)-1);  
    int mod_lower = some_random_i % *lower;
    int result_gen_num = (some_random_i % (*upper - *lower + 1)) + *lower;
    return result_gen_num;
}


int eat_func(void *arg)
{
    long num = (long)arg;
    int i;
    int time_eating = generate_simple_random(&min_of_range, &max_of_range);

    // while(!mutex_trylock(&mtx)) schedule();
    if (num == 1){
        while(!mutex_trylock(&mtx12) ) schedule();
        while(!mutex_trylock(&mtx51)) schedule();
        printk(KERN_ALERT "t_chinese ld - %d start eat. Please wait %d ms \n", num, time_eating);
        msleep(time_eating);
        printk(KERN_ALERT "t_chinese ld - %d stop eat \n", num);
        mutex_unlock(&mtx12);
        mutex_unlock(&mtx51);
    } else if (num == 2) {
        while(!mutex_trylock(&mtx23) ) schedule();
        while(!mutex_trylock(&mtx12)) schedule();
        printk(KERN_ALERT "t_chinese ld - %d start eat. Please wait %d ms \n", num, time_eating);
        msleep(time_eating);
        printk(KERN_ALERT "t_chinese ld - %d stop eat \n", num);
        mutex_unlock(&mtx12);
        mutex_unlock(&mtx23);
    } else if (num == 3) {
        while(!mutex_trylock(&mtx23)) schedule();
        while(!mutex_trylock(&mtx34)) schedule();
        printk(KERN_ALERT "t_chinese ld - %d start eat. Please wait %d ms \n", num, time_eating);
        msleep(time_eating);
        printk(KERN_ALERT "t_chinese ld - %d stop eat \n", num);
        mutex_unlock(&mtx34);
        mutex_unlock(&mtx23);
    } else if (num == 4) {
        while(!mutex_trylock(&mtx34)) schedule();
        while(!mutex_trylock(&mtx45)) schedule();
        printk(KERN_ALERT "t_chinese ld - %d start eat. Please wait %d ms \n", num, time_eating);
        msleep(time_eating);
        printk(KERN_ALERT "t_chinese ld - %d stop eat \n", num);
        mutex_unlock(&mtx34);
        mutex_unlock(&mtx45);
    } else if (num == 5) {
        while(!mutex_trylock(&mtx51) ) schedule();
        while(!mutex_trylock(&mtx45)) schedule();
        printk(KERN_ALERT "t_chinese ld - %d start eat. Please wait %d ms \n", num, time_eating);
        msleep(time_eating);
        printk(KERN_ALERT "t_chinese ld - %d stop eat \n", num);
        mutex_unlock(&mtx45);
        mutex_unlock(&mtx51);
    }

    // printk(KERN_ALERT "t_chinese_1 cannot starteat\n");
    // printk(KERN_ALERT "t_chinese ld - %d start eat \n", num);
    // msleep(90);
 

    // printk(KERN_ALERT "t_chinese ld - %d stop eat \n", num);
    // for (i=0; i<STEPS; i++)
    // {
    //     while(!mutex_trylock(&mtx)) schedule();
    //         if(i%10==0) printk(KERN_ALERT "%ld - %d\n", num, i);

    //         msleep(100);

    //         if(kthread_should_stop()) break;
    //         mutex_unlock(&mtx);
    //         msleep(90);
    // }

    // mutex_unlock(&mtx);

    return 0;
}

// struct hungry_chines

static struct hungry_chinese{
  // about kthread: https://embetronicx.com/tutorials/linux/device-drivers/linux-device-drivers-tutorial-kernel-thread/
  // https://metanit.com/c/tutorial/6.3.php
    int id; 
    char name[];
    // struct task_struct *etx_thread;
    // int size_misc;
    // int worker_count;
    // int* numbers;
    // int acc_sum;
};


int init_module(void)
{
    // hungry_chinese
    
    char t_chinese_1_name[] = "t_chinese_1";
    char t_chinese_2_name[] = "t_chinese_2";
    char t_chinese_3_name[] = "t_chinese_3";
    char t_chinese_4_name[] = "t_chinese_4";
    char t_chinese_5_name[] = "t_chinese_5";


    mutex_init(&mtx12);
    mutex_init(&mtx23);
    mutex_init(&mtx34);
    mutex_init(&mtx45);
    mutex_init(&mtx51);


    t_chinese_1 = kthread_create(eat_func, (void*)1, t_chinese_1_name);
    if(!t_chinese_1)
    {
        mutex_destroy(&mtx12);
        mutex_destroy(&mtx51);
        printk(KERN_ALERT "t_chinese_1 cannot start eat\n");
        return -ENOSYS;
    }


//just crate t_chinese_ action
    t_chinese_2 = kthread_create(eat_func, (void*)2, t_chinese_2_name);
    if(!t_chinese_2)
    {
        mutex_destroy(&mtx23);
        mutex_destroy(&mtx12);
        printk(KERN_ALERT "t_chinese_2 cannot starteat\n");
        return -ENOSYS;
    }

    t_chinese_3 = kthread_create(eat_func, (void*)3, t_chinese_3_name);
    if(!t_chinese_3)
    {
        mutex_destroy(&mtx23);
        mutex_destroy(&mtx34);
        printk(KERN_ALERT "t_chinese_3 cannot starteat\n");
        return -ENOSYS;
    }

    t_chinese_4 = kthread_create(eat_func, (void*)4, t_chinese_4_name);
    if(!t_chinese_4)
    {
        mutex_destroy(&mtx45);
        mutex_destroy(&mtx34);
        printk(KERN_ALERT "t_chinese_4 cannot starteat\n");
        return -ENOSYS;
    }

    t_chinese_5 = kthread_create(eat_func, (void*)5, t_chinese_5_name);
    if(!t_chinese_5)
    {
        mutex_destroy(&mtx51);
        mutex_destroy(&mtx45);
        printk(KERN_ALERT "t_chinese_5 cannot starteat\n");
        return -ENOSYS;
    }


//run thread
    get_task_struct(t_chinese_1);
    wake_up_process(t_chinese_1);
    
    get_task_struct(t_chinese_2);
    wake_up_process(t_chinese_2);

    get_task_struct(t_chinese_3);
    wake_up_process(t_chinese_3);

    get_task_struct(t_chinese_4);
    wake_up_process(t_chinese_4);

    get_task_struct(t_chinese_5);
    wake_up_process(t_chinese_5);


    printk(KERN_ALERT "threads started\n");

    return 0;
}

void cleanup_module(void)
{  
    kthread_stop(t_chinese_1);
    put_task_struct(t_chinese_1);

    kthread_stop(t_chinese_2);
    put_task_struct(t_chinese_2);

    kthread_stop(t_chinese_3);
    put_task_struct(t_chinese_3);

    kthread_stop(t_chinese_4);
    put_task_struct(t_chinese_4);

    kthread_stop(t_chinese_5);
    put_task_struct(t_chinese_5);

    printk(KERN_ALERT "threaddemo exited\n");
    // return 0;
}


MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");