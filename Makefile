current_dir := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

build-toolchain:
	docker buildx build --push --platform linux/arm64/v8,linux/amd64 --tag andreaaspesidev/concept-os-toolchain:latest .

launch-toolchain:
	docker run -t -i --privileged --name concept-os-toolchain -v $(current_dir):/concept-os  -v /dev/bus/usb:/dev/bus/usb andreaaspesidev/concept-os-toolchain

terminal:
	docker exec -it concept-os-toolchain /bin/bash --login
