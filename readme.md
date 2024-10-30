# Corrupted mementos

The program tracks its current state with sqlite

Mint tokens daily based on failed transactions number

Upload state to ipfs, for further decentralization


### Internals

`db` - state keeping with sqlite or postgres

`parser` - listens for new blocks, parses all transactions and saves failed to the `db`

`minter` - mints corrupted mementos based on `db` state

`core` - manages above processes and make sure that everything is correct and sync





