// https://www.opennet.ru/docs/RUS/zlp/005.html

#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <string.h>


#define FIFO_NAME "/dev/radio0"

#define BUFFER_SIZE	64

int main (int argc, char ** argv)
{
	int fd;
	ssize_t read_bytes;
	char buffer[BUFFER_SIZE+1];
  fd = open (FIFO_NAME, O_RDONLY);
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
	exit (0);
}
