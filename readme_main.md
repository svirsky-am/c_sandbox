
#Гайд -учебник
https://github.com/ttroy50/cmake-examples/tree/master/01-basic/A-hello-cmake



#хорошие практики cmake 
https://habr.com/ru/post/330902/


https://dreampuf.github.io/GraphvizOnline/

'c:\Program Files\Graphviz\bin\dot.exe' -Tpng  kos.dot -o output.png
& 'C:\Program Files\Graphviz\bin\dot' e:\projects\cmakeEdu\graphs\kos.dot -Tpng -o output.png

dot -Tps input.dot > output.eps
dot -Tpng input.dot > output.png


https://question-it.com/questions/4880065/cmake-statistika-kompiljatsii
https://stackoverflow.com/questions/24292898/compile-date-and-time-using-cmake


pvs-studio-analyzer credentials PVS-Studio Free FREE-FREE-FREE-FREE


/home/svirsky/.config/PVS-Studio/PVS-Studio.lic

    $ pvs-studio-analyzer trace -- make -j8
pvs-studio-analyzer analyze -j2 -l /home/svirsky/.config/PVS-Studio/PVS-Studio.lic -o PVS-Studio.log


mkdir ./.build && cd ./.build
cmake ../Step1

cmake --build .


 pvs-studio --license-info /home/svirsky/.config/PVS-Studio/PVS-Studio.lic

 pvs-studio-analyzer analyze -j2 -l /home/svirsky/.config/PVS-Studio/PVS-Studio.lic -o testRep.txt
  196  mkdir ./build && cd ./.build
  197  mkdir ./.build && cd ./.build
  198  cmake ../
  199  cmake --build ..



pvs-studio-analyzer analyze -c /usr/share/gcc  -j2 -o ./PVS_test-report.log -l /home/svirsky/.config/PVS-Studio/PVS-Studio.lic 


whereis gcc
 pvs-studio-analyzer trace -- make
 pvs-studio-analyzer analyze -j2 -o ./PVS_test-report.log

$ export PVS_LICENSE=~/prog/pvs/PVS-Studio.lic
$ export PVS_BIN=~/prog/pvs/PVS-Studio
 pvs-tool genconf  -l C++11 pvs.cfg


whereis gcc

plog-converter -a GA:1,2 -t tasklist -o report.tasks ./testRep.txt 


pvs-studio-analyzer analyze -l /path/to/PVS-Studio.lic \
                            -o /path/to/PVS-Studio.log \
                            -e /path/to/exclude-path \
                            -j<N>