#include <linux/module.h>

#define MOD_NAME "paramdemo"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

/*------------------ global variables start----------------*/

static int value1 = 1;
static int value2 = 1;
// static char *m_char = "empty";

/*----------------- declare module patams ------------------- */
/* two macroses  module_param() and MODULE_PARM_DESC() */

module_param(value1, int, 0664);
MODULE_PARM_DESC(value1, "module counter");

module_param(value2, int, 0664);
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

