# rom-untrim

This is a simple tool to untrim a ROM. Tested on one gba game, and nothing else.

## Usage
Replaces file in place, so make a backup first.
```
rom-untrim -f <file> -s <new-size> -p <padding-bytes>
```

## Building
### Debug
```
cargo build
```
### Release
```
cargo build --release
```