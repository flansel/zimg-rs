#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod ZIMG {
    pub mod FFI {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn build_graph() {
        unsafe {
            let mut src_fmt: ZIMG::FFI::zimg_image_format = mem::MaybeUninit::uninit().assume_init();
            let mut dst_fmt : ZIMG::FFI::zimg_image_format = mem::MaybeUninit::uninit().assume_init();
            ZIMG::FFI::zimg_image_format_default(&mut src_fmt, ZIMG::FFI::ZIMG_API_VERSION);
            ZIMG::FFI::zimg_image_format_default(&mut dst_fmt, ZIMG::FFI::ZIMG_API_VERSION);
            src_fmt.width = 1920;
            src_fmt.height = 1080;
            src_fmt.pixel_type = ZIMG::FFI::zimg_pixel_type_e::ZIMG_PIXEL_BYTE;
            src_fmt.subsample_w = 1;
            src_fmt.subsample_h = 1;
            src_fmt.color_family = ZIMG::FFI::zimg_color_family_e::ZIMG_COLOR_YUV;

            dst_fmt.width = 1920;
            dst_fmt.height = 1080;
            dst_fmt.pixel_type = ZIMG::FFI::zimg_pixel_type_e::ZIMG_PIXEL_BYTE;
            dst_fmt.subsample_w = 1;
            dst_fmt.subsample_h = 1;
            dst_fmt.color_family = ZIMG::FFI::zimg_color_family_e::ZIMG_COLOR_YUV;

            let graph = ZIMG::FFI::zimg_filter_graph_build(&src_fmt, &dst_fmt, std::ptr::null());
            assert!(!graph.is_null());
        }
    }
}