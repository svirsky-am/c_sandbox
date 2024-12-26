// https://www.opennet.ru/docs/RUS/zlp/005.html

#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <string.h>

#include <termios.h>

#define FIFO_NAME "/dev/fakerandom1"

#define BUFFER_SIZE	64

int main (int argc, char ** argv)
{
	int fd;
	ssize_t read_bytes;
	char buffer[BUFFER_SIZE+1];
  	fd = open (FIFO_NAME, O_RDONLY);
	int stop_flag = 0;

	char ch;
	  do
	// while(stop_flag == 0)
	{	
		sleep(1);

		if (fd < 0)
		{
			fprintf (stderr, "Cannot open file\n");
			exit (1);
		}	
		
		while ((read_bytes = read (fd, buffer, BUFFER_SIZE)) > 0)
		{

		printf("\n-----Print part by dev------\n");
			buffer[read_bytes] = 0; /* Null-terminator for C-string */
			fputs (buffer, stdout);
		}

		if (read_bytes < 0)
		{
			fprintf (stderr, "myread: Cannot read file\n");
			exit (1);
		}
		close (fd);
		// getchar() != 'q'
		ch = getchar();
		
	    printf("press [q] to quit\n");
	}while (ch != 'q');

  	exit (0);
}
