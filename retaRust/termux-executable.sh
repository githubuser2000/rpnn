[ "${PROFILE}" = "release" ] && for a in rreta rrp rrpl rrpe rrpb rgrundStrukHtml
do
	cargo run --release --bin $a -- -h
done || for a in rreta rrp rrpl rrpe rrpb rgrundStrukHtml
do
	cargo run --bin $a -- -h
done
echo "Build complete: $TARGET_DIR"
