
all:
	cargo build --release
	cp target/release/play_icfp2015 play_icfp2015

clean:
	cargo clean
	rm play_icfp2015
