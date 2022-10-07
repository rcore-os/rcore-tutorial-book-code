all: p1 p2 p3 p4

clean:
	rm -f p1 p2 p3 p4

p1:
	cargo run --bin p1

p2:
	cargo run --bin p2

p3:
	@cp src/bin/p3.rs ./
	cargo run --bin p3
	@rm ./p3.rs

p4:
	@cp src/bin/p4.rs ./
	cargo run --bin p4
	@mv ./p4.output ./src/bin
	@rm ./p4.rs
