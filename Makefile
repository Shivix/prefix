.PHONY: man
man:
	cargo build --release
	sed -i ':a;N;$$!ba;s/default: \n/default: \\\\n/' man/prefix.1
