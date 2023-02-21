# Twitch Helprrr

- [Contributing](#contributing)
  - [🍷 Wine](#-wine)
  - [Dependencies](#dependencies)
    - [binstall](#binstall)
    - [Toolchain 1.65.0](#toolchain-1650)
    - [Shuttle](#shuttle)
  - [Development server](#development-server)


## Contributing

> cargo install cargo-binstall, cargo binstall cargo-shuttle, rustup toolchain install 1.65.0, cargo +1.65.0 shuttle run
>
> -- <cite>esitsu</cite>

> we binstalled the space shuttle
>
> -- <cite>shaunen</cite>

> Don't forget to binstall that thang.
> you might get cargo finstalled if ya don't.
>
> -- <cite>wolfred</cite>

> here's my haiku cargo binstall shuttle cargo install space shuttle cargo unbinstall
>
> -- <cite>ErrorID107</cite>

> Error 107: cargo binstall not found
>
> -- <cite>esitsu</cite>

### 🍷 Wine

You will need wine for all of the installs of the binstalls.

### Dependencies

Rust version 1.65.0

#### binstall

    cargo install cargo-binstall

#### Toolchain 1.65.0

    rustup toolchain install 1.65.0

#### Shuttle

Binstall version 0.10.0:

    cargo-binstall cargo-shuttle


### Development server

Run the project:

    PORT=8686 cargo shuttle run
