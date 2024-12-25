#include <stdio.h>

#define FIFO_NAME "/dev/radio0"

int main ()
{


  int fd_in, fd_out;
  // char * nr_message = "Enter your name, please\n";
  char buf[64];
  fd_in = open("/dev/tty", O_RDONLY);
  // fd_out = open("/dev/tty", O_WRONLY);
  // write(fd_out, nr_message, strlen(nr_message));
  // while()
  do
  {
    read(fd_in, buf, 64);
    printf("Your name is %s\n", buf);
    ch = fgetc(f);
    putchar(ch);
  } while (ch != 'q');


  return EXIT_SUCCESS;

}


