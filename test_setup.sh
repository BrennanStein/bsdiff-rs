#!/usr/bin/env bash

# Compile C bsdiff-43
rm -rf target/c
mkdir target/c
cc bsdiff-43/bsdiff.c -DBSDIFF_EXECUTABLE -lbz2 -o target/c/bsdiff
cc bsdiff-43/bspatch.c -DBSPATCH_EXECUTABLE -lbz2 -o target/c/bspatch

# Compile Java jbsdiff
mvn -f jbsdiff/pom.xml clean package
rm -rf target/java
mkdir target/java
ln jbsdiff/target/jbsdiff-*.jar target/java/jbsdiff