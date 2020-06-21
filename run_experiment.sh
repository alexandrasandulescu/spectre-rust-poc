#! /bin/bash
CORE_VICTIM=1
CORE_ATTACKER=6
BINARY_PATH=`pwd`/target/debug

pipe=/tmp/rust-poc-pipe

rm -f $pipe
if [[ ! -p $pipe ]]; then
    mkfifo $pipe
fi

killall -9 dumb-victim attacker 2> /dev/null

taskset -c ${CORE_VICTIM} ${BINARY_PATH}/dumb-victim <$pipe &
victim_pid=$(pidof dumb-victim)

taskset -c ${CORE_ATTACKER} ${BINARY_PATH}/attacker >> experiment.result &

cat input >$pipe

wait
