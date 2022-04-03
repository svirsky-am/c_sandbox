int finalValueAfterOperations(char ** operations, int operationsSize){
    // printf("Hello finalValueAfterOperations! \n");
    unsigned int result=0;
    // int *addr=&result;
    for(int i=0; i< operationsSize; ++i)
    {
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
    }
    // printf("\nFinal result is %d" , result);
    return result;
}