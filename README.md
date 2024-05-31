# Overview
`undname` is a purely Rust-based implementation of a Microsoft symbol demangler. It functions as an alternative to [`msvc-demangler`](https://crates.io/crates/msvc-demangler) and Microsoft's own [`UnDecorateSymbolName`](https://learn.microsoft.com/en-us/windows/win32/api/dbghelp/nf-dbghelp-undecoratesymbolnamew). It is closely based off of LLVM's own [`llvm-undname`](https://github.com/llvm/llvm-project/tree/4985f25ffcc4735c36967fcdbd5d46e009b25827/llvm/tools/llvm-undname) and boasts competitive [performance](https://github.com/Ryan-rsm-McKenzie/undname-rs/tree/main/benches) and better accuracy when compared to existing implementations.

The latest development docs are available at:

The stable release docs are available at:

Changelogs are available at: https://github.com/Ryan-rsm-McKenzie/undname-rs/releases

# Example

```rust
use undname::Flags;
let result = undname::demangle(b"?world@@YA?AUhello@@XZ".into(), Flags::default()).unwrap();
assert_eq!(result, b"struct hello __cdecl world(void)"[..]);
```
