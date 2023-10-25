# RIP-8 - The Rust CHIP-8 emulator

Everyone's gotta write a CHIP-8 emulator. And in my case, it's gotta be in Rust ;)

# CHIP-8 keypad

The emulator uses the standard CHIP-8 keypad mapping on a modern keyboard. It also provides the `colemak` feature flag (use `cargo run --features colemak`) to emulate the CHIP-8 keypad correctly on the Colemak layout.

<table>
  <thead>
    <tr>
      <td>CHIP-8</td>
      <td>QWERTY</td>
      <td>Colemak</td>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>
        <table>
          <tbody>
            <tr>
              <td>1</td>
              <td>2</td>
              <td>3</td>
              <td>C</td>
            </tr>
            <tr>
              <td>4</td>
              <td>5</td>
              <td>6</td>
              <td>D</td>
            </tr>
            <tr>
              <td>7</td>
              <td>8</td>
              <td>9</td>
              <td>E</td>
            </tr>
            <tr>
              <td>A</td>
              <td>0</td>
              <td>B</td>
              <td>F</td>
            </tr>
          </tbody>
        </table>
      </td>
      <td>
        <table>
          <tbody>
            <tr>
              <td>1</td>
              <td>2</td>
              <td>3</td>
              <td>4</td>
            </tr>
            <tr>
              <td>Q</td>
              <td>W</td>
              <td>E</td>
              <td>R</td>
            </tr>
            <tr>
              <td>A</td>
              <td>S</td>
              <td>D</td>
              <td>F</td>
            </tr>
            <tr>
              <td>Z</td>
              <td>X</td>
              <td>C</td>
              <td>V</td>
            </tr>
          </tbody>
        </table>
      </td>
      <td>
        <table>
          <tbody>
            <tr>
              <td>1</td>
              <td>2</td>
              <td>3</td>
              <td>4</td>
            </tr>
            <tr>
              <td>Q</td>
              <td>W</td>
              <td>F</td>
              <td>P</td>
            </tr>
            <tr>
              <td>A</td>
              <td>R</td>
              <td>S</td>
              <td>T</td>
            </tr>
            <tr>
              <td>Z</td>
              <td>X</td>
              <td>C</td>
              <td>V</td>
            </tr>
          </tbody>
        </table>
      </td>
    </tr>
  </tbody>
</table>

# Testing the emulator

The `tests` directory contains [this CHIP-8 test suite](https://github.com/Timendus/chip8-test-suite), which can be used to verify the correctness of the implementation.

# License

This project is licensed under the [MIT License](/LICENSE). Do whatever you want with it.
