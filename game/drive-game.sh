#!/bin/bash
set -euxo pipefail
for (( ; ; ))
do
	cargo contract call\
		--url wss://ws.test.azero.dev\
		--contract "$1"\
		--suri "sting unfold aware abstract sponsor search size section priority twenty business couple"\
		--message submit_turn\
		--skip-dry-run\
		--gas 250000000000\
		--proof-size 512000\
		--skip-confirm \
		-x
done
