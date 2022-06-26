#!/bin/sh
I=0
while !(time -f :%e make run 1>build/run$I.log 2>build/run$I.err.log && grep panic build/run$I.log)
do
    echo $I no panic
    if !(grep :0. build/run$I.err.log || grep :1. build/run$I.err.log)
    then
        break
    fi
    I=$(expr $I + 1)
done
