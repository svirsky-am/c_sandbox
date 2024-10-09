
## Контакты 
isedykh@specialist.ru
/igor.v.sedykh



## книги

- гилбер шилдт
- кернмиган ритчи
- роберт лав - системное программироние ру

-  Daniel P. Bovet, Marco Cesati "Understanding the Linux Kernel, Third Edition" November 2005 ISBN 10: 0-596-00565-2
- https://man7.org/


## envirement
- build-essential
- libncurses6
- libncurses6-dev

## tips
- почитать про ошибки в errno
- int f(void) -нужно указывать void , тк.к это запрещает вызывать функцию с аргументами

- при линковке отбрасывается приставка lib от имени libworld.a (-lworld)
-lname 
-L. (показывает что надо использовать директрию "." для поиска статических библиотек)
- динамическая библиотека компилируется с ключом -shared (объкетники собираеются с ключом -fPIC - position independent code. Это компиляция с плавающими адерсами)

- 7 типов элментов файлоыой соситы(сокеты , файлы, папки, каналы , ссылки, символьные устройства , блочные устройства )
- cat/proc/envirement

- unistd (ssize_t)

- extern charr ** ?
- syscal()
- IO 0 1 2 (дескриптры stdin , stdout, stderr)

- ulimit -n (показать предельное количество дескриторов)
- буфер программмы равен размеру страницы памяти 
- bind - одностронняя привязка (не как bidirectional)
- 0_EXCL - не создваать новый файл и не удалять предыдущий
- fd (что такок файловый дескриптор ?)
- proc- инфа о процессах 
- struct flock (F_WRLCK, SEEK_SET)
- lseek



### day 2
- stuct dir entry описывает файл директории
- inode содержит служебную информацию о файле
- toch вызовает mk node
- sticky bit определяет тип nodeu
- IPC 
- pipe (парные каналыб , fifo - файлы )
- fork() - еджинственная функция  в линукс по созданию процессов
- массив int pipedes[2] (pipedes[0] - input, 1- output) IO

>                         pid>0   (parent)
> |main|->pipe->fork()->|     |->
>                     ->|     |->
>                        0 child


- dup2() - перенапраляет поток в другой поток по дескриптор. Наприемер, execve
- execve() - подменяет текущий процесс 
- fflush() - сброс потока
- mknod

- IPC-сообщение
    struct....
    {
        long mtype;
        ....
    }
    Что передается?
    Существует буфер сообщение (сообщения надо удалять из буфера)
    Есть возможность выбирать сообщения из очереди в соответсвии с их типом. 

- shared memory (shmem) выделяется с помощью shmget
- spin locks
### day3
- AF_UNIX- файловый сокет
- пасивные и активные сокеты (liasten - переводит сокет в режим активного ожижания)
```sh
edu_c/unix_c/block4_sockets/netserver 30333
telnet localhost 30333
```
порт освобождается через некоторые время

- sugnals
    1 SIGHUP 
    2 SIGINT interapt ctrl+c
    9 SIGKILL       kill -9 -1 
    15 SIGTERM      kill -15 1234
    19 SIGSTOP 

    SIGUSR1
    SIGUSR2
- процесс init позволяет удочеить зомби- проссы и собрать с них сигналы 



```sh
unix_c/block5_signals/sigdemo
# cltl+c
# cltl+z

# in pther terminal 
kill -15 8375
kill -1 8375
```


### day4

## utils

### nm
- nm ./a.out (просмотр адресного простравнтсва программы)
t - наши имена 
u -  сторонние функции 
- cntl - контроль  (fcntl - файл контроль)

### draw


### критерий работы ДЗ
./testlocks два раза 

./первый раз выходит после   get char
TODO нужнл снять блокировку для второо заупска 

block1


### home task day 2
1) написать на линуксе программу , которая переводит в чб монохромнынй файл bmp
2) задачник по логике и алгоритмам Корнева Валединский 2023
http://numa.math.msu.su/data/materials/Valedinsky_Kornev_MProgC_Release_V2.
3) создать свою файловую систему ( например блочную)


4) Написать брокер ips-сообщений (как вариант с форком)