#!/bin/bash

ps auwx | grep "rscsgo" | grep -v "grep" | grep "wyt" | awk '{print $2}' | xargs kill -9

echo "git pull code..."
git pull origin main

echo "start cargo build..."

cargo build --release

mv target/release/rscsgo ./rscsgo

nohup ./rscsgo > server.log 2>&1 &
echo "run rscsgo success"