#!/usr/bin/env bash

set -e

gkc="../target/release/gokartc"
gk="../target/release/gokart"
exe="exe.bin"
inp="input.txt"

# rebuild

cd ..
cargo build --release
cd bench

# bench function

do_bench () {
  local filename=$1
  local n=$2

  echo "> $filename"
  echo "n = $n"

  $gkc "$filename" -o $exe > /dev/null
  echo "$n" > $inp
  time $gk $exe < $inp > /dev/null
  echo ""
}

# tasks

do_bench "../tasks/task_01.gokart" "20"
do_bench "../tasks/task_02_rnd.gokart" "10000"
do_bench "../tasks/task_03_mut.gokart" "10000"

# cleanup

rm $exe
rm $inp
