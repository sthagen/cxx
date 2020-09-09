use crate::rust_string::RustString;
use crate::rust_vec::RustVec;
use std::mem;
use std::ptr;

macro_rules! rust_vec_shims {
    ($segment:expr, $ty:ty) => {
        const_assert_eq!(mem::size_of::<[usize; 3]>(), mem::size_of::<Vec<$ty>>());
        const_assert_eq!(mem::align_of::<usize>(), mem::align_of::<Vec<$ty>>());

        const _: () = {
            attr! {
                #[export_name = concat!("cxxbridge04$rust_vec$", $segment, "$new")]
                unsafe extern "C" fn __new(this: *mut RustVec<$ty>) {
                    ptr::write(this, RustVec { repr: Vec::new() });
                }
            }
            attr! {
                #[export_name = concat!("cxxbridge04$rust_vec$", $segment, "$drop")]
                unsafe extern "C" fn __drop(this: *mut RustVec<$ty>) {
                    ptr::drop_in_place(this);
                }
            }
            attr! {
                #[export_name = concat!("cxxbridge04$rust_vec$", $segment, "$len")]
                unsafe extern "C" fn __len(this: *const RustVec<$ty>) -> usize {
                    (*this).repr.len()
                }
            }
            attr! {
                #[export_name = concat!("cxxbridge04$rust_vec$", $segment, "$data")]
                unsafe extern "C" fn __data(this: *const RustVec<$ty>) -> *const $ty {
                    (*this).repr.as_ptr()
                }
            }
            attr! {
                #[export_name = concat!("cxxbridge04$rust_vec$", $segment, "$stride")]
                unsafe extern "C" fn __stride() -> usize {
                    mem::size_of::<$ty>()
                }
            }
        };
    };
}

macro_rules! rust_vec_shims_for_primitive {
    ($ty:ident) => {
        rust_vec_shims!(stringify!($ty), $ty);
    };
}

rust_vec_shims_for_primitive!(bool);
rust_vec_shims_for_primitive!(u8);
rust_vec_shims_for_primitive!(u16);
rust_vec_shims_for_primitive!(u32);
rust_vec_shims_for_primitive!(u64);
rust_vec_shims_for_primitive!(i8);
rust_vec_shims_for_primitive!(i16);
rust_vec_shims_for_primitive!(i32);
rust_vec_shims_for_primitive!(i64);
rust_vec_shims_for_primitive!(f32);
rust_vec_shims_for_primitive!(f64);

rust_vec_shims!("string", RustString);
