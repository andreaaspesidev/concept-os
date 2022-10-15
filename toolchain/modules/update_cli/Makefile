build:
	cargo build --release --features uart
	cp ./target/release/update-cli ./update-cli-uart
	chmod +x ./update-cli-uart
	cargo build --release --features mqtt
	cp ./target/release/update-cli ./update-cli-mqtt
	chmod +x ./update-cli-mqtt

bluetooth_connect:
	sudo rfcomm connect /dev/rfcomm0 98:D3:32:70:A0:CA 1 
	# 00:22:02:01:46:45