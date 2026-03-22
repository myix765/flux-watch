.PHONY: wasm dev clean

wasm:
	cd core && wasm-pack build --target bundler --out-dir ../ui/src/lib/wasm

dev: wasm
	cd ui && pnpm dev

clean:
	cd core && cargo clean
	rm -rf ui/src/lib/wasm