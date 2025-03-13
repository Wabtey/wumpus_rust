# Wumpus in Rust

The Wumpus game is used to train AI agent with artificial sensation (`Breeze`, `Stench`, `Sparkling`).
The goal is to move across the map without falling into a pit or get eaten by the Wumpus.

## Features

- [x] Map Setup
- [x] Tile Status AutoApplication
- [x] Map Display in CLI

    ```text
    --------------------------------------------------------
    |             | S           |             | B           |
    | A           |             |             |             |
    --------------------------------------------------------
    | S           | B           | B S Sk      |             |
    |             | W           | G           | P           |
    --------------------------------------------------------
    | B           | S           | B           | B           |
    |             | P           |             |             |
    --------------------------------------------------------
    |             | B           |             |             |
    |             |             |             |             |
    --------------------------------------------------------
    ```

- [ ] Mechanism
  - [ ] move
  - [ ] lose conditions
  - [ ] win condition

## Quick Start

- `cargo run` with cargo 1.85.0-nightly (should also work with any toolchain but not tested)
