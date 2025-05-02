#![no_main]

use coverage_formats::jacoco::JacocoReport;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = JacocoReport::from_read(&mut &data.to_vec()[..]);

    // fuzzed code goes here
});
