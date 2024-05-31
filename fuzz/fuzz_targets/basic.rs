#![no_main]

use libfuzzer_sys::fuzz_target;
use undname::Flags;

fuzz_target!(|data: &[u8]| {
    _ = undname::demangle(data.into(), Flags::default());
});
