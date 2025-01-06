# Курсы по СИ

## C basic
- [Linux. Уровень 2. Программирование в Linux на C](https://www.specialist.ru/course/unsi)
- [Конспект лекций Си базового уровня](unix_c/notes_c_basic.md)


## C kernel drivers
 - [Разработка драйверов устройств в Linux](https://www.specialist.ru/course/unsis)
 - [Конспект лекций Си драйверов Linux](unix_c_drivers/notes_c_drivers.md)

Автор: `ivsedykh@mephi.24`

#  PVS
```sh
pvs-studio-analyzer credentials PVS-Studio Free FREE-FREE-FREE-FREE
/home/svirsky/.config/PVS-Studio/PVS-Studio.lic
# show license info
pvs-studio --license-info /home/svirsky/.config/PVS-Studio/
pvs-studio-analyzer trace -- make -j8
pvs-studio-analyzer analyze -j2 -l /home/svirsky/.config/PVS-Studio/PVS-Studio.lic -o PVS-Studio.log

plog-converter -a GA:1,2 -t tasklist -o report.tasks ./
```


```shell
pvs-studio-analyzer analyze -l /path/to/PVS-Studio.lic \
                            -o /path/to/PVS-Studio.log \
                            -e /path/to/exclude-path \
                            -j<N>
```



# Ссылки
- [полезные рецепты по сборке проектов на Си](https://github.com/ttroy50/cmake-examples/tree/master/01-basic/A-hello-cmake)
- [Построение графов зависимостей онлайн](https://dreampuf.github.io/GraphvizOnline/)


https://question-it.com/questions/4880065/cmake-statistika-kompiljatsii

