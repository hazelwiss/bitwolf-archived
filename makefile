.PHONY: quick
quick:
	cargo run -- --load-rom nds:/home/nibble/Downloads/roms/NDS/arm7wrestler.nds

clean_gen:
	rm -rf core/gen