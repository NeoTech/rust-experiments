#!/bin/env bash
RUSTFLAGS='-C link-args=-s' cargo build -r --bin spotify