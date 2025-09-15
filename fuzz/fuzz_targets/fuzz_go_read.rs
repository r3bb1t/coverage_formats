#![no_main]

use coverage_formats::go::GoReport;
use libfuzzer_sys::fuzz_target;

use std::cell::UnsafeCell;

fuzz_target!(|data: &[u8]| {
    // It's a fuzz test, i know that it's safe to use UnsafeCell here
    let data_unsafecell = UnsafeCell::new(data);
    // Safety: it's safe since each time fuzz function executes,
    // there is only one owner of th data
    let mut_data_ref = unsafe { &mut (*data_unsafecell.get()) };
    let _ = GoReport::from_buf_read(mut_data_ref);
});
