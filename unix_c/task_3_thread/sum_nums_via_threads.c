#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <pthread.h>







int numberCount(FILE* input) {
  fseek(input, 0, SEEK_SET);
  int counter = 0;
  while (1) {
    int value;
    if (fscanf(input, "%d", &value) == 1)
      counter++;
    if (feof(input))
      break;
  }
  return counter;
}

void read_numbers(FILE* input, int size, int* numbers) {
  fseek(input, 0, SEEK_SET);
  for (int i = 0; i < size; ++i) {
    fscanf(input, "%d", &numbers[i]);
  }
}

void print_array(int* numbers, int size) {
  for (int i = 0; i < size; ++i) {
    printf("%d ; ", numbers[i]);
  }
  printf("\n");
}

struct meta_of_array{
  // https://metanit.com/c/tutorial/6.3.php
    int size;
    int worker_count;
    int* numbers;
    int acc_sum;
};

void * thread_func(void *arg)
{
  struct meta_of_array * p_meta_of_array =arg;

  int size = (*p_meta_of_array).size;
  int i;
  printf("Size %d\n", size);
  int worker_id =(*p_meta_of_array).worker_count;
  p_meta_of_array->worker_count += 1;
   printf("worker_id %d, size %d, \n", worker_id,  size);
   for(i = worker_id; i < size; i+=2) 
   {
    printf(" %d - ", p_meta_of_array->numbers[i]);
    p_meta_of_array->acc_sum += p_meta_of_array->numbers[i];
   };

}



int main(int argc, char * argv[])
{
  FILE* input = fopen("INPUT.TXT", "r");
  if (input == 0) {
    printf("cant open file\n");
    return 1;
  }
  int size = numberCount(input);
  int* numbers = (int*)malloc(sizeof(int) * size);
  read_numbers(input, size, numbers);
  print_array(numbers, size);



   int  result;
   pthread_t thread1, thread2;

   struct meta_of_array meta_arr;
   meta_arr.acc_sum = 0;
   meta_arr.numbers = numbers;
   meta_arr.size = size;
   meta_arr.worker_count = 0;



  // Run first worker
   result = pthread_create(&thread1, NULL, thread_func,  &meta_arr);
   if (result != 0) {
     perror("Creating the first thread");
     return EXIT_FAILURE;
   }
  // Run second worker
  
   result = pthread_create(&thread2, NULL, thread_func, &meta_arr );
   if (result != 0) {
     perror("Creating the second thread");
     return EXIT_FAILURE;
   }
   result = pthread_join(thread1, NULL);
   if (result != 0) {
     perror("Joining the first thread");
     return EXIT_FAILURE;
   }
   result = pthread_join(thread2, NULL);
   if (result != 0) {
     perror("Joining the second thread");
     return EXIT_FAILURE;
   }
  
   printf("\n\nResult: %d\n", meta_arr.acc_sum);
   printf("Done\n");

   return EXIT_SUCCESS;
}
