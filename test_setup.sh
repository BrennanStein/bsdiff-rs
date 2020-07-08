#!/usr/bin/env bash
rm -rf target/c
mkdir target/c
cc bsdiff-43/bsdiff.c -DBSDIFF_EXECUTABLE -lbz2 -o target/c/bsdiff
cc bsdiff-43/bspatch.c -DBSPATCH_EXECUTABLE -lbz2 -o target/c/bspatch