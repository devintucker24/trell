#!/usr/bin/env zsh
set -e

source ./env.zsh

cargo run -- "$1"

llc \
  -mtriple=arm64-apple-macosx15.2 \
  -filetype=obj \
  out.ll \
  -o trell-program.o

xcrun clang trell-program.o -o trell-program

./trell-program
