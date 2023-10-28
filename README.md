# RIP-8 - The Rust CHIP-8 interpreter

Everyone's gotta write a CHIP-8 interpreter. And in my case, it's gotta be in Rust ;)

# Usage

You can run a CHIP-8 ROM on the interpreter by passing it as an argument on the command line:

```sh
cargo run -- /path/to/rom
# or
cargo build --release
target/rip8 /path/to/rom
```
For starters, there are some fun games present in the `roms` folder, along with the classic IBM logo program. Personal recommendation - play Tetris :D

# CHIP-8 keypad

The interpreter uses the standard CHIP-8 keypad mapping on a modern keyboard. It also provides the `colemak` feature flag (use `cargo run --features colemak`) to emulate the CHIP-8 keypad correctly on the Colemak layout.

<table align="center">
  <thead>
    <tr>
      <td align="center"><b>CHIP-8</b></td>
      <td align="center"><b>QWERTY</b></td>
      <td align="center"><b>Colemak</b></td>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>
        <table>
          <tbody>
            <tr>
              <td><tt>1</tt></td>
              <td><tt>2</tt></td>
              <td><tt>3</tt></td>
              <td><tt>C</tt></td>
            </tr>
            <tr>
              <td><tt>4</tt></td>
              <td><tt>5</tt></td>
              <td><tt>6</tt></td>
              <td><tt>D</tt></td>
            </tr>
            <tr>
              <td><tt>7</tt></td>
              <td><tt>8</tt></td>
              <td><tt>9</tt></td>
              <td><tt>E</tt></td>
            </tr>
            <tr>
              <td><tt>A</tt></td>
              <td><tt>0</tt></td>
              <td><tt>B</tt></td>
              <td><tt>F</tt></td>
            </tr>
          </tbody>
        </table>
      </td>
      <td>
        <table>
          <tbody>
            <tr>
              <td><tt>1</tt></td>
              <td><tt>2</tt></td>
              <td><tt>3</tt></td>
              <td><tt>4</tt></td>
            </tr>
            <tr>
              <td><tt>Q</tt></td>
              <td><tt>W</tt></td>
              <td><tt>E</tt></td>
              <td><tt>R</tt></td>
            </tr>
            <tr>
              <td><tt>A</tt></td>
              <td><tt>S</tt></td>
              <td><tt>D</tt></td>
              <td><tt>F</tt></td>
            </tr>
            <tr>
              <td><tt>Z</tt></td>
              <td><tt>X</tt></td>
              <td><tt>C</tt></td>
              <td><tt>V</tt></td>
            </tr>
          </tbody>
        </table>
      </td>
      <td>
        <table>
          <tbody>
            <tr>
              <td><tt>1</tt></td>
              <td><tt>2</tt></td>
              <td><tt>3</tt></td>
              <td><tt>4</tt></td>
            </tr>
            <tr>
              <td><tt>Q</tt></td>
              <td><tt>W</tt></td>
              <td><tt>F</tt></td>
              <td><tt>P</tt></td>
            </tr>
            <tr>
              <td><tt>A</tt></td>
              <td><tt>R</tt></td>
              <td><tt>S</tt></td>
              <td><tt>T</tt></td>
            </tr>
            <tr>
              <td><tt>Z</tt></td>
              <td><tt>X</tt></td>
              <td><tt>C</tt></td>
              <td><tt>V</tt></td>
            </tr>
          </tbody>
        </table>
      </td>
    </tr>
  </tbody>
</table>

# Testing

The `tests` directory contains [this CHIP-8 test suite](https://github.com/Timendus/chip8-test-suite), which can be used to verify the correctness of the implementation.


# License

This project is licensed under the [MIT License](/LICENSE). Do whatever you want with it.
