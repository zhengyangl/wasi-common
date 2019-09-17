//! Types and constants specific to 32-bit wasi. These are similar to the types
//! in the `host` module, but pointers and `usize` values are replaced with
//! `u32`-sized types.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::wasi::*;

// C types
pub(crate) type long = i32;
pub(crate) type ulong = u32;

// libc types
pub(crate) type size_t = ulong;
pub(crate) type intptr_t = long;
pub(crate) type uintptr_t = ulong;
pub(crate) type timer_t = uintptr_t; // *mut ::std::os::raw::c_void
pub(crate) type caddr_t = uintptr_t; // *mut i8

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct __wasi_ciovec_t {
    pub buf: uintptr_t, // *const ::std::os::raw::c_void
    pub buf_len: size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct __wasi_iovec_t {
    pub buf: uintptr_t, // *mut ::std::os::raw::c_void
    pub buf_len: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct __wasi_prestat_t {
    pub pr_type: __wasi_preopentype_t,
    pub u: __wasi_prestat_t___wasi_prestat_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union __wasi_prestat_t___wasi_prestat_u {
    pub dir: __wasi_prestat_t___wasi_prestat_u___wasi_prestat_u_dir_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct __wasi_prestat_t___wasi_prestat_u___wasi_prestat_u_dir_t {
    pub pr_name_len: size_t,
}

pub(crate) const INTPTR_MIN: i32 = -2147483648;
pub(crate) const INTPTR_MAX: u32 = 2147483647;
pub(crate) const UINTPTR_MAX: u32 = 4294967295;
pub(crate) const PTRDIFF_MIN: i32 = -2147483648;
pub(crate) const PTRDIFF_MAX: u32 = 2147483647;
pub(crate) const SIG_ATOMIC_MIN: i32 = -2147483648;
pub(crate) const SIG_ATOMIC_MAX: u32 = 2147483647;
pub(crate) const SIZE_MAX: u32 = 4294967295;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bindgen_test_layout_wasi_ciovec_t() {
        assert_eq!(
            ::std::mem::size_of::<__wasi_ciovec_t>(),
            8usize,
            concat!("Size of: ", stringify!(__wasi_ciovec_t))
        );
        assert_eq!(
            ::std::mem::align_of::<__wasi_ciovec_t>(),
            4usize,
            concat!("Alignment of ", stringify!(__wasi_ciovec_t))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_ciovec_t>())).buf as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_ciovec_t),
                "::",
                stringify!(buf)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_ciovec_t>())).buf_len as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_ciovec_t),
                "::",
                stringify!(buf_len)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_wasi_iovec_t() {
        assert_eq!(
            ::std::mem::size_of::<__wasi_iovec_t>(),
            8usize,
            concat!("Size of: ", stringify!(__wasi_iovec_t))
        );
        assert_eq!(
            ::std::mem::align_of::<__wasi_iovec_t>(),
            4usize,
            concat!("Alignment of ", stringify!(__wasi_iovec_t))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_iovec_t>())).buf as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_iovec_t),
                "::",
                stringify!(buf)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_iovec_t>())).buf_len as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_iovec_t),
                "::",
                stringify!(buf_len)
            )
        );
    }

    #[test]
    fn bindgen_test_layout___wasi_prestat_t___wasi_prestat_u___wasi_prestat_u_dir_t() {
        assert_eq!(
            ::std::mem::size_of::<__wasi_prestat_t___wasi_prestat_u___wasi_prestat_u_dir_t>(),
            4usize,
            concat!(
                "Size of: ",
                stringify!(__wasi_prestat_t___wasi_prestat_u___wasi_prestat_u_dir_t)
            )
        );
        assert_eq!(
            ::std::mem::align_of::<__wasi_prestat_t___wasi_prestat_u___wasi_prestat_u_dir_t>(),
            4usize,
            concat!(
                "Alignment of ",
                stringify!(__wasi_prestat_t___wasi_prestat_u___wasi_prestat_u_dir_t)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<__wasi_prestat_t___wasi_prestat_u___wasi_prestat_u_dir_t>()))
                    .pr_name_len as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_prestat_t___wasi_prestat_u___wasi_prestat_u_dir_t),
                "::",
                stringify!(pr_name_len)
            )
        );
    }

    #[test]
    fn bindgen_test_layout___wasi_prestat_t___wasi_prestat_u() {
        assert_eq!(
            ::std::mem::size_of::<__wasi_prestat_t___wasi_prestat_u>(),
            4usize,
            concat!("Size of: ", stringify!(__wasi_prestat_t___wasi_prestat_u))
        );
        assert_eq!(
            ::std::mem::align_of::<__wasi_prestat_t___wasi_prestat_u>(),
            4usize,
            concat!(
                "Alignment of ",
                stringify!(__wasi_prestat_t___wasi_prestat_u)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<__wasi_prestat_t___wasi_prestat_u>())).dir as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_prestat_t___wasi_prestat_u),
                "::",
                stringify!(dir)
            )
        );
    }

    #[test]
    fn bindgen_test_layout___wasi_prestat_t() {
        assert_eq!(
            ::std::mem::size_of::<__wasi_prestat_t>(),
            8usize,
            concat!("Size of: ", stringify!(__wasi_prestat_t))
        );
        assert_eq!(
            ::std::mem::align_of::<__wasi_prestat_t>(),
            4usize,
            concat!("Alignment of ", stringify!(__wasi_prestat_t))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_prestat_t>())).pr_type as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_prestat_t),
                "::",
                stringify!(pr_type)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_prestat_t>())).u as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_prestat_t),
                "::",
                stringify!(u)
            )
        );
    }
}
