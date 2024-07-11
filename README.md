# Onigiri (Formerly "Senzubean") Frontend

This repo consists of the Onigiri frontend using the Bevy engine in rust and integration with a simple torii client implementation.

## Instructions

First, make sure to setup and run the necessary cairo smart contracts via the [cairo-senzubean](https://github.com/elton-cs/cairo-senzubean) repo. The front end will only work when run in conjunction with the cairo smart contract backends (which includes all the setups of katana, torii, etc...)

Then, simply run the binary using the following command setting: 
```
cargo run
```

note: you might need to make sure you have all the necessary dependencies needed to run Bevy. Tested working with Linux and Windows under WSL2.