write WIT world manually like a caveman

go to /moonbit-guest/

```
wit-bindgen moonbit --out-dir . ../wit --derive-show --derive-eq
```

⚠️ NOTE! This will destroy the stubs you wrote

write the stubs

then run
```
moon build --target wasm
```

then run

```
wasm-tools component embed --world calculator ../wit _build/wasm/release/build/gen/gen.wasm -o embedded.wasm --encoding utf16
```

```
wasm-tools component new embedded.wasm -o component.wasm
```

```
cd ../
cargo run
```

---



```
winget install --id Git.Git -e --source winget;
winget install -e --id Microsoft.VisualStudioCode;
winget install --id Casey.Just --exact
```