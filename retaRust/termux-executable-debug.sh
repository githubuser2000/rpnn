for a in rreta  rgrundStrukHtml
do
	cargo run --features="rust-tool-bins" --bin $a -- -h
done
for a in rrp rrpl rrpe rrpb
do
	cargo run --features="rust-frontends" --bin $a -- -h
done
