
// https://leetcode.com/problems/final-value-of-variable-after-performing-operations/
#include <stdio.h>
#include <string.h>

int finalValueAfterOperations(char ** operations, int operationsSize){
    printf("Hello finalValueAfterOperations! \n");
    unsigned int result=0;
    int *addr_result=&result;
  

    for(int i=1; i< operationsSize; i++)
    {
        int command =(int)operations[i][1];
        switch ((int)operations[i][1])
        {
        case 43:
            /* code */
            ++result;
            break;       
        case 45:
            /* code */
            --result;
            break;
        default:
            break;
        }
        printf("\ncommand %d\n" ,  command);
        
    }
    printf("\nFinal result is %d" , result);
    return result;
}

int test(int argc, char ** argv){
    printf("Hello test 2 test world! \n");
    // printf("%s", *operations); 
    int i;
    for(i=1; i< argc; i++)
        printf("[%s]", argv[i]);
    return 0;

}