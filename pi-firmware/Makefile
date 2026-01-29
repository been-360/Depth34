release: cross upload

upload: 
	scp.exe ./target/aarch64-unknown-linux-gnu/release/pi-firmware been@rpi.local:/home/been/Depth34/pi-firmware

image:
	docker build -t cross-pi-udev .

cross:
	cross build --release --target aarch64-unknown-linux-gnu

cc:
	docker run --rm -it \
		-v ~/FlowTek/Depth34/pi-firmware:/project \
		-w /project \
		aarch64-compiler \
		cargo build --release --target aarch64-unknown-linux-gnu
