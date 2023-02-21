# Twitch Helprrr

- [Contributing](#contributing)
  - [Dependencies](#dependencies)
    - [binstall](#binstall)
    - [Toolchain 1.65.0](#toolchain-1650)
    - [Shuttle](#shuttle)
  - [Development server](#development-server)


## Contributing

### Dependencies

Rust version 1.65.0

#### binstall

    cargo install cargo-binstall

#### Toolchain 1.65.0

    rustup toolchain install 1.65.0

#### Shuttle

Binstall version 0.10.0:

    cargo-binstall shuttle


### Development server

Run the project:

    PORT=8686 cargo shuttle run
