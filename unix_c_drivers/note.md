#day1 
```sh
sudo apt install gcc
sudo apt install build-essential 
sudo apt install build-essential 
sudo apt install libncurses6
sudo apt install libncurses-dev
sudo apt install flex
sudo apt install bison


 
```
```sh
cd /usr/src
 sudo tar -xvf ./linux-source-6.8.0.tar.bz2 
```
```
cd /usr/src/linux-source-6.8.0
```


Output by kernel 
```cpp
printk()
```
watch
```sh
journalxtl
#or
tail -f /var/log/kern.log
```

load module
```
sudo insmod ./hello.ko
```
```
sudo rmmod ./hello.ko
```

## paramdemo 
modinfo ./paramdemo.ko
```sh
sudo insmod paramdemo.ko m_cout=20 m_char='"test1 test2"'
 sudo insmod .build/paramdemo.ko m_cout=20 m_char='"test1 test2"'

modinfo ./paramdemo.ko
```

```sh
ls /sys/module/ | grep paramdemo
```
show params
```sh
cat /sys/module/paramdemo/parameters/*
```
## paramdemo2
```sh
modinfo .build/paramdemo.ko
sudo insmod .build/paramdemo.ko value1=31 value2=32

```

```sh
sudo /sys/module/paramdemo/parameters/value2
sudo su
echo 33 >  /sys/module/paramdemo/parameters/value2 
```
## task_1_random_generator
```
sudo insmod .build/paramdemo.ko min_of_range=31 max_of_range=32
```
