# sized-data-derive

Procedural macro implementation for `sized-data` crate, used with Solana's Anchor framework.

## Implementation

Automatically derives the `SizedData` trait for structs by:
1. Analyzing struct fields at compile time
2. Generating size calculation code for each field
3. Producing a total size implementation

```rust
#[derive(SizedData)]
pub struct UserAccount {
    pub authority: Pubkey,    // 32 bytes
    pub counter: u64,         // 8 bytes
}

// Generates:
impl SizedData for UserAccount {
    fn size() -> usize {
        <Pubkey as SizedData>::size() + 
        <u64 as SizedData>::size()
    }
}
```

## Supported Field Types

- Named fields (`struct Example { field: Type }`)
- Unnamed fields (`struct Example(Type)`)
- Unit structs (`struct Example;`)

## Requirements

- Rust 1.83.0+
- quote = "1.0"
- syn = { version = "1.0", features = ["full"] }

## Usage

This crate is typically used as a dependency of `sized-data`. Direct usage:

```toml
[dependencies]
sized_data_derive = "0.1.0"
```

```rust
use sized_data_derive::SizedData;
```

## License

MIT License