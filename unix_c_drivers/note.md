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

 sudo insmod .build/paramdemo.ko m_cout=20 m_char='"test1 test2"'

```