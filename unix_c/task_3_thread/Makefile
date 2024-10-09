all:  sum_nums_via_threads

rebuild_and_run: clean sum_nums_via_threads
	./sum_nums_via_threads

sum_nums_via_threads:
	gcc sum_nums_via_threads.c -D_REENTERANT -o sum_nums_via_threads  -lpthread -O0
	strip sum_nums_via_threads
install:
	cp sum_nums_via_threads ~/bin
clean:
	 rm -f *.o   
	 rm -f sum_nums_via_threads
	 rm -f *~
