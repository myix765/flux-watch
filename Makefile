.PHONY: wasm dev clean

wasm:
	cd core && wasm-pack build --target web --out-dir ../ui/src/lib/wasmc

dev: wasm
	cd ui && pnpm dev

clean:
	cd core && cargo clean
	rm -rf ui/src/lib/wasm