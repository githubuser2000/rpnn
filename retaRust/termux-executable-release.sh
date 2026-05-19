for a in rreta  rgrundStrukHtml
do
	cargo run --release --features="rust-tool-bins" --bin $a -- -h
done
for a in rrp rrpl rrpe rrpb
do
	cargo run --release --features="rust-frontends" --bin $a -- -h
done
