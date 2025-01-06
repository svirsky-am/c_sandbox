#include <linux/module.h>
// #include <stdlib.h>

#define MOD_NAME "summator_of_random_sums_dev"
#define DEV_NAME "summator_dev_demo"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

/*------------------ global variables start----------------*/

static int in_array_len = 5;
static int summator_dev_count = 0;
static int min_of_range = 1;
static int max_of_range = 10;
static struct file_operations fops;
static int summator_dev_major;

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

int summator_dev_open(struct inode *inode, struct  file *file) 
{
    if(summator_dev_count > 0) return -EBUSY;
    summator_dev_count++;


    int result_massive[in_array_len];
    int sum_acc = 0;
    for (int i=0; i<in_array_len; ++i)
    {
        result_massive[i] = generate_simple_random(&min_of_range, &max_of_range);
        pr_info("for i %d generate %d", i, result_massive[i]);
        sum_acc += result_massive[i];
    }
    pr_info("result sum = %d", sum_acc);
    return 0;
}

int summator_dev_release(struct inode *inode, struct  file *file)
{
    summator_dev_count--;
    printk(KERN_ALERT " released\n");
    return 0;
}

/*---------------------- value set func--------------------------------*/

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
/* two macroses  module_param() and MODULE_PARM_DESC() */



module_param(in_array_len, int, 0664);
MODULE_PARM_DESC(in_array_len, "Len of input array: in_array_len");

module_param_cb(min_of_range, &kpops, &min_of_range, 0664);
MODULE_PARM_DESC(min_of_range, "module counter min_of_range");

module_param_cb(max_of_range, &kpops, &max_of_range, 0664);
MODULE_PARM_DESC(max_of_range, "module counter max_of_range");






int init_module()
{   
    memset(&fops, 0, sizeof(fops));
    fops.owner = THIS_MODULE;
    fops.open = summator_dev_open;
    fops.release = summator_dev_release;
    

    summator_dev_major = register_chrdev(0, DEV_NAME, &fops);
    if(summator_dev_major < 0)
    {
        printk(KERN_ALERT DEV_NAME " failed to register\n");
        return summator_dev_major;
    }

    printk(KERN_ALERT MOD_NAME "min_of_range  = %d max_of_range = %d\n", min_of_range, max_of_range);
    printk(KERN_ALERT DEV_NAME " registered , major number = %d\n", summator_dev_major);

    return 0;

}


void cleanup_module(void)
{
    printk(KERN_ALERT MOD_NAME " exited\n");
    return;
}




/*------------------ global variables and ----------------*/
