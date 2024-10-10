#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <pthread.h>

pthread_mutex_t condition_mutex = PTHREAD_MUTEX_INITIALIZER;


#define NTHREADS 8


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
    int pool_of_workers;
};

void * thread_func(void *arg)
{
  struct meta_of_array * p_meta_of_array =arg;

  int size = (*p_meta_of_array).size;
  int i;
  int inc_for_loop = (*p_meta_of_array).pool_of_workers;

  printf("Size %d\n", size);
  int worker_id =(*p_meta_of_array).worker_count;
  p_meta_of_array->worker_count += 1;
   printf("worker_id %d, size %d, \n", worker_id,  size);
   for(i = worker_id; i < size; i+=inc_for_loop) 
   {
    printf(" %d - ", p_meta_of_array->numbers[i]);
    pthread_mutex_lock( &condition_mutex );
    p_meta_of_array->acc_sum += p_meta_of_array->numbers[i];
    pthread_mutex_unlock( &condition_mutex );
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

  pthread_t thread_id[NTHREADS];

  struct meta_of_array meta_arr;
  meta_arr.acc_sum = 0;
  meta_arr.numbers = numbers;
  meta_arr.size = size;
  meta_arr.worker_count = 0;
  meta_arr.pool_of_workers = NTHREADS;

  int i, j;


  for(i=0; i < NTHREADS; i++)
  {
    pthread_create( &thread_id[i], NULL, thread_func, &meta_arr );
    printf("Creating the thread number %d...\n", i);
  }

  for(j=0; j < NTHREADS; j++)
  {
    pthread_join( thread_id[j], NULL);
    printf("Joining the thread number %d...\n", j);
  }

  
   printf("\n\nResult: %d\n", meta_arr.acc_sum);
   printf("Done\n");

   return EXIT_SUCCESS;
}
