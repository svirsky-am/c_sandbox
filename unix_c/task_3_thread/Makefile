all:  threads 
threads:
	gcc threads.c -D_REENTERANT -o threads  -lpthread -O0
	strip threads
install:
	cp threads ~/bin
clean:
	 rm -f *.o   
	 rm -f threads
	 rm -f *~
