#!/bin/bash

ps auwx | grep "csgo-item-price" | grep -v "grep" | grep "wyt" | awk '{print $2}' | xargs kill -9

echo "git pull code..."
git pull origin main

echo "start cargo build..."

cargo build --release

mv target/release/csgo-item-price ./csgo-item-price

nohup ./csgo-item-price > server.log 2>&1 &
echo "run csgo-item-price success"