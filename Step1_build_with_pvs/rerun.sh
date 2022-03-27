#!/usr/bin/bash
set -ex
export SCRIPT_DIR=$PWD
export BUILD_DIR=".build"
rm -rf $BUILD_DIR && mkdir $BUILD_DIR && cd $BUILD_DIR
cmake ../
pvs-studio-analyzer trace -- make 
# cmake --build .
pvs-studio-analyzer analyze  -j2 -o ./PVS_test-report.log  
plog-converter -a GA:1,2 -t tasklist -o report.tasks ./PVS_test-report.log 

# pvs-studio-analyzer analyze -l /home/svirsky/.config/PVS-Studio/PVS-Studio.lic \
#                             -o ./PVS-Studio.log \
#                             -j3