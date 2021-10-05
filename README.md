[![crates.io](https://img.shields.io/crates/v/typeless?style=flat-square)](https://crates.io/crates/typeless)
[![build status](https://img.shields.io/github/workflow/status/Cyborus04/typeless/Rust?style=flat-square)](https://github.com/Cyborus04/typeless/actions)

# typeless

`unsafe` API for type erasure on the stack

## Usage

Add the following to your `Cargo.toml`

```toml
[dependencies]
typeless = "0.1"
```

Storing any value `x` of type `T` in `TypeErased` completely destroys all type data associated
with it.

## Restrictions

While this erases all type data, leaving only the pure bytes, the compiler still requires 2
things:

- Size: The size of a `TypeErased` is not based on the data it contains, but
  rather a const generic parameter `C`, effectively a "maximum size" on the types it can contain.

- Alignment: Until there is a way to define alignment by a const parameter, the alignment of `TypeErased` is
  8 bytes, so anything with an alignment of 8 or less can be contained

### Access

Since there is no type data anymore, any access to the inner data is `unsafe` (except [getting the bytes directly](crate::TypeErased::raw))

`TypeErased` is not `Send` nor `Sync` since it can't be known if that's safe

## License

This project is licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

- [MIT License](http://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
