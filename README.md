#hexdump

A crate for creating simple hex dumps.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
hexdump = "0.1"
```

## Example

```rust
let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];
println!("{}", hexdump::formatted_dump_string(0x2000, &data));
```      

Output:

```
00002000  00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f ................
```

