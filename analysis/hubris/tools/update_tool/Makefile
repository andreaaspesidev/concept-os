current_dir := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

build:
	CARGO_TARGET_DIR="$(current_dir)/target" cargo build --release --features uart
	cp ./target/release/update-tool ./update-tool-uart
	chmod +x ./update-tool-uart
	CARGO_TARGET_DIR="$(current_dir)/target" cargo build --release --features mqtt
	cp ./target/release/update-tool ./update-tool-mqtt
	chmod +x ./update-tool-mqtt

bluetooth_connect:
	sudo rfcomm connect /dev/rfcomm0 98:D3:32:70:A0:CA 1 
	# 00:22:02:01:46:45