// This is an independent project of an individual developer. Dear PVS-Studio, please check it.

// PVS-Studio Static Code Analyzer for C, C++, C#, and Java: https://pvs-studio.com
// #include <iostream>

// #include "finalValueAfterOperations.h"
#include "2_twoSum.h"
// #include "src/2_twoSum.h"

// #include "solution.h"

int main(int argc, char *argv[])
{
    printf("Hello test world! \n");
    // finalValueAfterOperations('x', 1);
    int i;
    for(i=1; i< argc; i++)
        printf("%i[%s], ", i, argv[i]);
    // test(1, 'x');

    // // test(argc, argv);
    // i = finalValueAfterOperations(argv, argc);
    // printf("\ni is %d" , i);
    
    int test_nums[7] = {1, 2, 3, 4, 5 ,6 ,7};

    // out = twoSum(int* nums, int numsSize, int target, int* returnSize)

    // int* out; 
    int* testPoint;

    int testPoint2;
    // testPoint2 = twoSum(test_nums, 6, 7, testPoint);
    int test_nums2 = 8;
    testPoint2 = test2(test_nums2);
    printf("%d", testPoint2);

    // printf("%d", *out);

    return 0;
}