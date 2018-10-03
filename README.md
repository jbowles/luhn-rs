# Luhn
Validate credit card numbers. Implementations for the luhn algorithm accepting both string or digits. Still a work in progress, not useable as a library yet.

## Example

Two examples `Luhn::lun()` for `&str` or `u64`.

```rust
    let ex1 = "4539 1488 0343 6467";
    let ex2 = 4539148803436467;

    //true
    println!("ex1(true): {} for '{}'", Luhn::luhn(ex1.to_string()), ex1);
    println!("ex2(true): {} for '{}'", Luhn::luhn(ex2), ex2);
```