# Custom NPC Editor 2

Custom NPC Editor for elona_omake_overhaul.

## Installation

Download packaged binaries from the [releases page](https://github.com/Seacolor/cnpc2/releases/latest).

## Building

### Prerequisites

* Rust
* Node.js
* npm
* WebView

### Install tauri

<pre>
$ cargo install tauri-cli
$ npm install -g yarn
</pre>

### Debug

<pre>
$ yarn
$ yarn build
$ yarn tauri dev
</pre>

### Build

<pre>
$ yarn tauri build
</pre>

## Test

<pre>
$ cd src-tauri
$ cargo test
</pre>

## License

MIT License