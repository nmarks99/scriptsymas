#!/usr/bin/env bash

cargo new $1
cp -r config/hidden_cargo/ $1/.cargo
cp config/Cargo.toml $1/ 
cd $1
rustup override set nightly-2021-01-07

