# Savage Metronome

[![CI](https://github.com/p3tri3/savage_metronome/actions/workflows/ci.yml/badge.svg)](https://github.com/p3tri3/savage_metronome/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/p3tri3/savage_metronome/graph/badge.svg)](https://codecov.io/gh/p3tri3/savage_metronome)
[![CodeQL](https://github.com/p3tri3/savage_metronome/actions/workflows/codeql.yml/badge.svg)](https://github.com/p3tri3/savage_metronome/actions/workflows/codeql.yml)

A primitive metronome desktop application with features such as:
- Tap tempo
- Optional visual cue (for beat timing)
- Sound pitch and duration adjustment controls (from click to beep)
- Equal temperament tuning (reference pitch, octave, and note selection)
- Simple preset saving

<img src="assets/savage_metronome.png" alt="Savage Metronome" width="300"/>

## Screenshot

![Screenshot](doc/screenshot.png)

## Usage

### Windows

1.  Build the project:

    ``` bash
    cargo build --release
    ```

2.  Run the application:

    ``` bash
    cargo run --release
    ```

### MacOS

1. To generate the .app bundle:

    ``` bash
    cargo install cargo-bundle
    ```

    ``` bash
    cargo bundle --release
    ```

## Testing

``` bash
cargo test                    # Run all tests (unit + integration)
cargo test --no-default-features  # Run tests with mock audio (no rodio)
```

Unit tests live inside source files (`#[cfg(test)]` modules). Integration tests are in `tests/`. The mock audio backend allows the audio engine to be tested without a real audio device.

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.
