#!/bin/bash
count=500
binary="target/release/foo"

echo "Creating $count processes with parent memory of 5MB, 50MB, 100MB, 250MB, 500MB, 750MB, 1000MB"

echo "fork-exec"
for m in 5 50 100 250 500 750 1000;
do
    t="$(./target/release/process_create_compare -b $binary -c $count -w 0 -m $m 2>&1 > /dev/null)"
    echo $t
done

echo "command"
for m in 5 50 100 250 500 750 1000;
do
    t="$(./target/release/process_create_compare -b $binary -c $count -w 3 -m $m 2>&1 > /dev/null)"
    echo $t
done
