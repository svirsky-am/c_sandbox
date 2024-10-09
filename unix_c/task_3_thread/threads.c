#include <stdlib.h>
#include <stdio.h>
#include <errno.h>
#include <pthread.h>

// void inline swap(int *i, int *j)
void swap(int *i, int *j)
{
    int t;
    t = *i;
    *i = *j;
    *j = t;
}

void reverse(int * v, int n)
{
   int i;
   for (i = 0; i < (n/2); i++)
     swap(&v[i], &v[n-1-i]);
}

int next_permutation(int * v, int n)
{
   int i, j;
   i = n - 1;
   while ((i > 1) && (v[i] < v[i-1])) i--;
   if (v[i] > v[i-1]) {
     j = n - 1;
     while (v[j] < v[i-1]) j--;
     swap(&v[j], &v[i-1]);
     reverse(&v[i], n-i);
     return 1;
   }
   return 0;
}

void print_vect(int * v, int n)
{
   int i;
   for (i = 0; i < n - 1; i++)
     printf("%i ", v[i]);
   printf("%i\n", v[n-1]);
}

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
    int size;
    int worker_count;
    int* numbers;
    int acc_sum;
};

void * thread_func(void *arg)
{
  int size = * (int *) arg;
  printf("Size %d\n", size);
  int i;
  int worker_id = * ((int * ) arg + 1);
  // worker_id ++;
  
   
  // //  Приведение типа (умножение указателя на int)
  //  int size = * (int *) arg; 
  //     v = malloc(sizeof(int)*size);
  //  for(i = 0; i < size; i++) v[i] = i+1;
   printf("Size %d, worker_id %d\n", size, worker_id);
  //  print_vect(v, size);

  // //  print_array(arg, size);

  //  printf("Size %d\n", size);
  // //  while(next_permutation(v, size)) {
  // //    print_vect(v, size);
  // //    sync();
  // //  }
  //  free(v);
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



  //  size = 4;
   result = pthread_create(&thread1, NULL, thread_func,  &meta_arr);
   if (result != 0) {
     perror("Creating the first thread");
     return EXIT_FAILURE;
   }
  //  size2 = 5;



   
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
   printf("Done\n");
   return EXIT_SUCCESS;
}
