# aqueren

## Dependencies
`apt-get install libssl-dev`

## Run
`cargo run --bin aqueren`
`cargo run --bin client`

## Useful developing tools
`cargo install cargo-watch`
`cargo watch build`
`cargo watch test`

## API
Get the game state, GET /state
Place a tile. POST /action with the following body format
`{ player: 1, tile: { row: 1, col: 2 } }`
