COMPONENT_NAME:=component1
current_dir := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

dump-cbf:
	../../../../../libs/cbf_lite/tests/simple_read/target/release/cbf_simple_read $(current_dir)/$(COMPONENT_NAME).cbf