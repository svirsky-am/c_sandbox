#include <linux/module.h>
// #include <stdlib.h>

#define MOD_NAME "generate_random"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

/*------------------ global variables start----------------*/

static int min_of_range = 1;
static int max_of_range = 10;
/*---------------------- value set func--------------------------------*/

static int value_set(const char *val, const struct kernel_param *kp )
{   
    int old = max_of_range;
    int ret, res;

    ret = kstrtoint(val, 10, &res);
    if(ret != 0 || res < 0 || res > 100) return -EINVAL;

    param_set_int(val, kp);

    printk(KERN_ALERT MOD_NAME " old value = %d, new value = %d\n", old, max_of_range);
    return 0;
};


/*---------------------- callback structure--------------------------------*/

static const struct kernel_param_ops kpops = {
    .set = value_set, 
    .get = param_get_int
};


/*----------------- declare module patams ------------------- */
/* two macroses  module_param() and MODULE_PARM_DESC() */



module_param(min_of_range, int, 0664);
MODULE_PARM_DESC(min_of_range, "module counter min_of_range");

module_param_cb(max_of_range, &kpops, &max_of_range, 0664);
MODULE_PARM_DESC(max_of_range, "module counter max_of_range");


int generate_simple_random(int *lower, int *upper)
{
    unsigned int some_random_i;
    get_random_bytes(&some_random_i, sizeof(some_random_i)-1);  
    int mod_lower = some_random_i % *lower;
    int num = (some_random_i % (*upper - *lower + 1)) + *lower;
    printk(KERN_ALERT MOD_NAME "\tmod_lower = %d, random number = %d \tresult = %d", mod_lower, some_random_i, num);
    

    return num;
}



int init_module()
{
    printk(KERN_ALERT MOD_NAME "min_of_range  = %d max_of_range = %d\n", min_of_range, max_of_range);
    int mun = generate_simple_random(&min_of_range, &max_of_range);
    printk(KERN_ALERT MOD_NAME "random number = %d ", mun);
    return 0;
}


void cleanup_module(void)
{
    printk(KERN_ALERT MOD_NAME " exited\n");
    return;
}




/*------------------ global variables and ----------------*/
