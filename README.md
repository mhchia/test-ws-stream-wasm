# test-ws-stream-wasm

This is a reproduction for `ws-stream-wasm` failing with nightly and rust rustflags `+atomics,+bulk-memory,+mutable-globals`.

My environment:
- OS: macOS 12.6
- cargo: 1.68.0-nightly (70898e522 2022-12-05)
- rustc: 1.68.0-nightly (bdb07a8ec 2022-12-11)
- Chrome: Version 116.0.5845.140 (Official Build) (x86_64)

# Step 1: Clone the repo
```bash
git clone
```

# Step 1: Use nightly rust
Creating a file `rust-toolchain` with the content `nightly-2022-12-12` under tlsn-extension project root.
```bash
echo "nightly-2022-12-12" > rust-toolchain
```

## Step 2: Use rustflags `+atomics,+bulk-memory,+mutable-globals`

Add a file `.cargo/config` under tlsn-extension project root with the following content, as wasm-bindgen-rayon [suggested](https://github.com/GoogleChromeLabs/wasm-bindgen-rayon#using-config-files).

```bash
[target.wasm32-unknown-unknown]
rustflags = ["-C", "target-feature=+atomics,+bulk-memory,+mutable-globals"]

[unstable]
build-std = ["panic_abort", "std"]
```


## Step 3: Start a websocket server
Open a terminal and run the following commands to start a websocket server, which prints the messages received from the client.

```bash
cd ws-server
yarn install
yarn start
```

## Step 4: Build wasm and run a web server
Open another terminal and run

```bash
yarn build-and-start
```

Open the page `http://localhost:8080` in your browser. A websocket from `ws-stream-wasm` then try sending a message to the websocket server from Step 3. It should **fail** with the following error message.

![Alt text](image.png)

## Step 5: Remove the file `.cargo/config` and try again
Remove the file `.cargo/config`, run `yarn build-and-start` again, and open the page `http://localhost:8080` in your browser. It should **succeed**, and you can see `Successfully sent message to server` in your browser console.
