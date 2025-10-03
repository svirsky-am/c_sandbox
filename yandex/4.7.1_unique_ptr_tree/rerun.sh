#!/usr/bin/bash

# https://new.contest.yandex.ru/contests/42114/problem?id=40119%2F2022_10_29%2Fa5LFlLHdXT

set -ex
export SCRIPT_DIR=$PWD
export BUILD_DIR=".build"
rm -rf $BUILD_DIR && mkdir $BUILD_DIR && cd $BUILD_DIR
cmake ../
cmake --build .

 ./4.7.1_share_pointer 


# pvs-studio-analyzer trace -- make 
# pvs-studio-analyzer analyze  -j2 -o ./PVS_test-report.log  
# plog-converter -a GA:1,2 -t tasklist -o report.tasks ./PVS_test-report.log 

