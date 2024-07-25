```bash
➜  swc-ecma-preset-env-error cargo build
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling swc_core v0.99.4
error[E0432]: unresolved import `preset_env_base`
  --> /.cargo/registry/src/index.crates.io-6f17d22bba15001f/swc_core-0.99.4/src/lib.rs:38:13
   |
38 |     pub use preset_env_base::*;
   |             ^^^^^^^^^^^^^^^ use of undeclared crate or module `preset_env_base`

For more information about this error, try `rustc --explain E0432`.
error: could not compile `swc_core` (lib) due to 1 previous error
```