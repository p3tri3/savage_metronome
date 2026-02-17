# Savage Metronome

A primitive metronome desktop application with features such as:
- Tap tempo
- Optional visual cue (for beat timing)
- Sound pitch and duration adjustment controls (from click to beep)
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

## License

This project is licensed under the MIT License.

Copyright (c) 2026

Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be included
in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
