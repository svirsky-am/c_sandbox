#include <linux/module.h>
// #include <stdlib.h>

#define MOD_NAME "generate_random"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("svirsky");

/*-----------------------lib func----------------*/

void clear_acc(char *str)
{
    int s;
    // for (s= str; *s; ++s){
    //     &str[s] = 0;
    // };
    // return (s);
}

int calculate_len(char *str)
{
    const char *s;
    for (s= str; *s; ++s);
    return (s- str);
}

int calculate_sun_of_array(int *len_of_array, int *nums_array)
{
    int count_of_sum_in_stack = *len_of_array;
    // int size_of_nums_array = calculate_len(nums_array);
    int size_of_nums_array = *len_of_array;
    int acc_sum = 0;

    printk(KERN_ALERT MOD_NAME "\t*len_of_array = %d array: %s size_of_nums_array: %d", *len_of_array, nums_array, size_of_nums_array);
    char acc_substr_num[32];

    // printk(KERN_ALERT MOD_NAME "\t*acc_substr_num before clean: %s", *acc_substr_num);
    // clear_acc(acc_substr_num);
    // printk(KERN_ALERT MOD_NAME "\t*acc_substr_num after clean: %s", *acc_substr_num);

    for (int i = 0; i < size_of_nums_array; ++i) {
        printk(" \tnum %d: %d", i, &nums_array[i]);
    }
    // print(while count_of_sum_in_stack != 0 )
    
    // for()
    // printk(KERN_ALERT MOD_NAME "\t*len_of_array = %d, random number = %d \tresult = %d", mod_lower, some_random_i, num);

    return *len_of_array;
}

/*------------------ global variables start----------------*/

static int in_array_len = 3;
static int nums_array[] = { 3, 4, 5 };
/*---------------------- value set func--------------------------------*/

static int value_set(const char *val, const struct kernel_param *kp )
{   
    int old = nums_array;
    int ret, res;

    ret = kstrtoint(val, 10, &res);
    if(ret != 0 || res < 0 || res > 100) return -EINVAL;

    param_set_int(val, kp);

    // printk(KERN_ALERT MOD_NAME " old value = %d, new value = %d\n", old, nums_array);
    // int sum_of_arr = calculate_sun_of_array(&in_array_len, &nums_array);
    // printk(KERN_ALERT MOD_NAME "reseted random number = %d ", sum_of_arr);
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

module_param_array(nums_array, int, NULL, 0664);
MODULE_PARM_DESC(nums_array, "Array of numbers: nums_array");






int init_module()
{
    printk(KERN_ALERT MOD_NAME "in_array_len  = %d nums_array = \n", in_array_len );
    int sum_of_arr = calculate_sun_of_array(&in_array_len, nums_array);
    printk(KERN_ALERT MOD_NAME "random number = %d ", sum_of_arr);
    return 0;
}


void cleanup_module(void)
{
    printk(KERN_ALERT MOD_NAME " exited\n");
    return;
}




/*------------------ global variables and ----------------*/
