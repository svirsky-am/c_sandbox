# all: msgdemo polymsgdemo shmemdemo semdemo

all: clean polymsgdemo
# msgdemo:
# 	gcc msgserv.c -o msgserv
# 	gcc msgcli.c -o msgcli

binary: polymsgserv.o libpolymsg.a
	gcc -o binary polymsgserv.o -L. -lpolymsg

polymsgdemo:
	gcc polymsgserv.c -o polymsgserv
	gcc polymsgcli.c -o polymsgcli


polymsgserv.o:
	gcc polymsgserv.c -g -o polymsgserv

libpolymsg.a: libpolymsg.o
	ar cr libpolymsg.a libpolymsg.o

libpolymsg.o:
	gcc libpolymsg.c -g -o libpolymsg.o



# shmemdemo:
# 	gcc shmemserv.c -o shmemserv
# 	gcc shmemcli.c -o shmemcli
# semdemo:
# 	gcc semserv.c -o semserv
# 	gcc semcli.c -o semcli
# # install:
# 	cp msgserv ~/bin
# 	cp msgcli ~/bin
# 	cp polymsgserv ~/bin
# 	cp polymsgcli ~/bin
# 	cp shmemserv ~/bin
# 	cp shmemcli ~/bin
# 	cp semserv ~/bin
# 	cp semcli ~/bin
clean:
	 rm -f *.o *.a   
	 rm -f *serv *cli *msg
	 rm -f *~