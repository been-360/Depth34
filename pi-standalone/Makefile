release: cross upload

upload: 
	scp.exe ./target/aarch64-unknown-linux-gnu/release/controller been@rpi.local:/home/been/Depth34/controller

image:
	docker build -t cross-pi-udev .

cross:
	cross build --release --target aarch64-unknown-linux-gnu

cc:
	docker run --rm -it \
		-v ~/FlowTek/Depth34/controller:/project \
		-w /project \
		aarch64-compiler \
		cargo build --release --target aarch64-unknown-linux-gnu
