all:  sum_nums_via_threads

rebuild_and_run: clean sum_nums_via_threads
	./sum_nums_via_threads

sum_nums_via_threads:
	gcc sum_nums_via_threads.c -D_REENTERANT -o sum_nums_via_threads  -lpthread -O0
	strip sum_nums_via_threads

rebuild_and_run_mutex: clean sum_nums_via_threads_run_mutex
	./sum_nums_via_threads_mutex


# /home/user/edu_c/unix_c/task_3_thread/sum_nums_via_threads_mutex.c
sum_nums_via_threads_run_mutex:
	gcc sum_nums_via_threads_mutex.c -D_REENTERANT -o sum_nums_via_threads_mutex  -lpthread -O0
	strip sum_nums_via_threads_mutex

install:
	cp sum_nums_via_threads ~/bin
clean:
	 rm -f *.o   
	 rm -f sum_nums_via_threads sum_nums_via_threads_mutex
	 rm -f *~
