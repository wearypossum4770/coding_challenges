#!/bin/bash

read -a NUMBERS
COUNTER=0
for number in "${NUMBERS[@]}"
do
if [ $number -gt 0 ]
then
    COUNTER=$(( $COUNTER + $number ))
else 
    COUNTER=$(( $COUNTER - $number ))
fi
done
   echo $COUNTER
# array=($(echo $string| tr " " "\n"))
# for number in ; do 
# if  (($number -gt 0 ));
# then 
# COUNTER = COUNTER+number
# else 
# COUNTER=COUNTER-number
# fi
# echo $COUNTER
# done