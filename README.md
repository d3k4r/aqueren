# aqueren

## Install deps

```
apt-get install libssl-dev

# If you like
cargo install cargo-watch
```

## Install other deps, compile, and run

`cargo run`

Also useful:

`cargo watch build`
`cargo watch test`

## API
Place a tile. POST /action with the following body format
`{ player: 1, tile: { row: 1, col: 2 } }`
