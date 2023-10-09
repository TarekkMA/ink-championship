# Squink Splash!

Compete against the players on the grid of various size to colour as many cells as possible 
while using the least amount of gas.

## Setup

By default the page will connect to the Aleph Zero Testnet network. 

1. Compile the contract using `cargo contract build --release`
2. Here are the instantiation parameters and what they mean:
   1. `dimensions` - Dimension of a grid.
   2. `buyIn` - how many tokens each player must provide as a value to a call in order to join.
   3. `formingRounds` - how many blocks should before we can start a game.
   4. `rounds` - number of rounds in blocks
3. Open a dApp by running the [frontend locally](README.md#running-the-frontend-locally)
4. Share the game contract address and metadata 
(metadata file can be found in [complete_contracts folder](/complete_contracts/)) with players.
5. Players should register their contracts by adding On-Chain contract in contracts-ui and calling `registerPlayer` message.
6. After forming rounds passed, start game by executing `startGame` message.
7. Add `SURI` of the account with which you instantiated the game to the environment variable of shell.
8. Run `drive-game.sh` script with a contract address argument: `./drive-game.sh <game_address>`
9. Enjoy!

## Running the Frontend locally

Navigate to [fronted folder](/game/frontend/) and run

```bash
yarn
yarn dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser.

An example game can be found using this address (5D6eZ7LyfypYPPYr9iJiBNKtUr6roTSf1Di4SoDN7wtWamCK). 
Enter it into the input on the home page.

