update:
	rm -rf 	./src/flash_allocator
	mkdir 	./src/flash_allocator
	cp -R 	../../src/*		./src/flash_allocator/
	mv ./src/flash_allocator/lib.rs ./src/flash_allocator/mod.rs
	sed -i 's/crate/crate::flash_allocator/g' ./src/flash_allocator/*
	
coverage:
	cargo clean
	cargo tarpaulin --out HTML
	xdg-open tarpaulin-report.html

clean:
	cargo clean

test:
	cargo test