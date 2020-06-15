#! /bin/bash
CORE_VICTIM=1
CORE_ATTACKER=6

pipe=/tmp/rust-poc-pipe

rm -f $pipe
if [[ ! -p $pipe ]]; then
    mkfifo $pipe
fi

killall -9 dumb-victim attacker 2> /dev/null

taskset -c ${CORE_VICTIM} ./dumb-victim <$pipe &
victim_pid=$(pidof dumb-victim)

sleep 1
taskset -c ${CORE_ATTACKER} ./attacker >> experiment.result &

cat input >$pipe

wait
