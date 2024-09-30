#!/bin/bash

DAY=$1
cargo init day-$DAY
cd day-$DAY
rm -rf .git
cd ..
git add day-$DAY
git commit -m "Add day $DAY"