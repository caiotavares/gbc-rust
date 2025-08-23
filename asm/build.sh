#!/bin/bash

has_extension=$(echo $1 | grep '\.')

if [[ $has_extension != "" ]]; then
    filename=$(echo $1 | cut -d '.' -f1)
else 
    filename=$1
fi

if [[ $filename == "" ]]; then
    echo "Specify a .asm file to compile!"
    exit 1
fi

rgbasm -o $filename.o $filename.asm
rgblink -o $filename.gb $filename.o
rgbfix -v -p 0xFF $filename.gb
