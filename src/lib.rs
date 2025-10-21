#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn build_graph() {
        unsafe {
            let mut src_fmt: zimg_image_format = mem::MaybeUninit::uninit().assume_init();
            let mut dst_fmt : zimg_image_format = mem::MaybeUninit::uninit().assume_init();
            zimg_image_format_default(&mut src_fmt, ZIMG_API_VERSION);
            zimg_image_format_default(&mut dst_fmt, ZIMG_API_VERSION);

            let graph = zimg_filter_graph_build(&src_fmt, &dst_fmt, std::ptr::null());
            assert!(graph.is_null());
        }
    }
}