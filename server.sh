#!/bin/bash

program="/home/lzw/.cargo/bin/cairo-language-server"
#/home/lzw/bin/heaptrack-v1.2.0-x86_64.AppImage $program "$@"
heaptrack $program "$@"
