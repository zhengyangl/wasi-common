#![allow(non_camel_case_types)]
#![allow(unused_unsafe)]
#![allow(unused)]
use crate::memory::*;
use crate::sys::host_impl;
use crate::{wasi, wasi32, Result};

use wasi_common_cbindgen::wasi_common_cbindgen;

pub(crate) fn clock_res_get(clock_id: wasi::__wasi_clockid_t) -> Result<wasi::__wasi_timestamp_t> {
    unimplemented!("clock_res_get")
}

pub(crate) fn clock_time_get(clock_id: wasi::__wasi_clockid_t) -> Result<wasi::__wasi_timestamp_t> {
    unimplemented!("clock_time_get")
}

pub(crate) fn poll_oneoff(
    input: Vec<Result<wasi::__wasi_subscription_t>>,
    output_slice: &mut [wasi::__wasi_event_t],
) -> Result<wasi32::size_t> {
    unimplemented!("poll_oneoff")
}
