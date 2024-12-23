#include <linux/module.h>

#define MOD_NAME "paramdemo"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

/*------------------ global variables start----------------*/

static int value1 = 1;
static int value2 = 1;
/*---------------------- value set func--------------------------------*/

static int value_set(const char *val, const struct kernel_param *kp )
{   
    int old = value2;
    int ret, res;

    ret = kstrtoint(val, 10, &res);
    if(ret != 0 || res < 0 || res > 100) return -EINVAL;

    param_set_int(val, kp);

    printk(KERN_ALERT MOD_NAME " old value = %d, new value = %d\n", old, value2);
    return 0;
};


/*---------------------- callback structure--------------------------------*/

static const struct kernel_param_ops kpops = {
    .set = value_set, 
    .get = param_get_int
};


/*----------------- declare module patams ------------------- */
/* two macroses  module_param() and MODULE_PARM_DESC() */

module_param(value1, int, 0664);
MODULE_PARM_DESC(value1, "module counter");

module_param_cb(value2, &kpops, &value2, 0664);
MODULE_PARM_DESC(value2, "module string");

int init_module()
{
    printk(KERN_ALERT MOD_NAME "value1  = %d value2 = %d\n", value1, value2);
    return 0;
}


void cleanup_module(void)
{
    printk(KERN_ALERT MOD_NAME " exited\n");
    return;
}




/*------------------ global variables and ----------------*/

