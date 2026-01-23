release: cross upload

upload: 
	scp.exe ./target/aarch64-unknown-linux-gnu/release/controller been@rpi.local:/home/been/Depth34/controller

image:
	docker build -t cross-pi-udev .

cross:
	cross build --release --target aarch64-unknown-linux-gnu