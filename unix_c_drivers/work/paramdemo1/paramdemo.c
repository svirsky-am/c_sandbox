#include <linux/module.h>

#define MOD_NAME "paramdemo"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

/*------------------ global variables start----------------*/

static int m_count = 1;
static char *m_char = "empty";

/*----------------- declare module patams ------------------- */
/* two macroses  module_param() and MODULE_PARM_DESC() */

module_param(m_count, int, S_IRUSR|S_IWUSR|S_IRGRP|S_IWGRP|S_IROTH);
MODULE_PARM_DESC(m_coubt, "module counter");

module_param(m_char, charp, S_IRUSR|S_IWUSR|S_IRGRP|S_IWGRP|S_IROTH);
MODULE_PARM_DESC(m_coubt, "module string");

int init_module()
{
    if (m_count == 1 ) printk(KERN_ALERT MOD_NAME " using default counter\n;");
    else printk(KERN_ALERT MOD_NAME "using string %.*s\n", m_count, m_char);
    return 0;
}


void cleanup_module(void)
{
    printk(KERN_ALERT MOD_NAME " exited\n");
    return;
}




/*------------------ global variables and ----------------*/

