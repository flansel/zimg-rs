

pub mod ZIMG {
    use crate::ZIMG::FFI::{zimg_image_buffer, zimg_image_buffer_const, zimg_image_format, ZIMG_API_VERSION};

    pub mod FFI {
        #![allow(non_upper_case_globals)]
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }
}

#[cfg(test)]
mod tests {
    use std::alloc::{alloc, Layout};
    use std::ffi::{c_uint, c_void};
    use super::*;
    use crate::ZIMG::FFI::{zimg_filter_graph_process, zimg_image_buffer_const, zimg_image_buffer, zimg_image_format, zimg_filter_graph_get_tmp_size, ZIMG_API_VERSION, zimg_error_code_e, ZIMG_BUFFER_MAX};

    #[test]
    fn build_graph() {
        let mut src_fmt : zimg_image_format = Default::default();
        let mut dst_fmt : zimg_image_format = Default::default();
        unsafe {
            ZIMG::FFI::zimg_image_format_default(&mut src_fmt, ZIMG::FFI::ZIMG_API_VERSION);
            ZIMG::FFI::zimg_image_format_default(&mut dst_fmt, ZIMG::FFI::ZIMG_API_VERSION);
        }

        src_fmt.width = 1920;
        src_fmt.height = 1080;
        src_fmt.pixel_type = ZIMG::FFI::zimg_pixel_type_e::ZIMG_PIXEL_BYTE;
        src_fmt.subsample_w = 0;
        src_fmt.subsample_h = 0;
        src_fmt.color_family = ZIMG::FFI::zimg_color_family_e::ZIMG_COLOR_YUV;
        src_fmt.alpha = ZIMG::FFI::zimg_alpha_type_e::ZIMG_ALPHA_NONE;

        dst_fmt.width = 1280;
        dst_fmt.height = 720;
        dst_fmt.pixel_type = ZIMG::FFI::zimg_pixel_type_e::ZIMG_PIXEL_BYTE;
        dst_fmt.subsample_w = 0;
        dst_fmt.subsample_h = 0;
        dst_fmt.color_family = ZIMG::FFI::zimg_color_family_e::ZIMG_COLOR_YUV;
        dst_fmt.alpha = ZIMG::FFI::zimg_alpha_type_e::ZIMG_ALPHA_NONE;

        let graph = unsafe { ZIMG::FFI::zimg_filter_graph_build(&src_fmt, &dst_fmt, std::ptr::null()) };
        assert!(!graph.is_null());

        let src_buffer_size = (src_fmt.height * src_fmt.width * 3) as usize;
        let layout = Layout::from_size_align(src_buffer_size, 32).unwrap();
        let src_buffer = unsafe { alloc(layout) };

        let mut src : zimg_image_buffer_const = Default::default();
        src.version = ZIMG_API_VERSION;
        src.plane[0].data = src_buffer as *mut c_void;
        src.plane[0].stride = (src_fmt.width) as isize;
        src.plane[0].mask = c_uint::MAX;
        src.plane[1].data = unsafe { src_buffer.offset((src_fmt.height * src_fmt.width) as isize) } as *mut c_void;
        src.plane[1].stride = (src_fmt.width) as isize;
        src.plane[1].mask = c_uint::MAX;
        src.plane[2].data = unsafe { src_buffer.offset((src_fmt.height * src_fmt.width * 2) as isize) } as *mut c_void;
        src.plane[2].stride = (src_fmt.width) as isize;
        src.plane[2].mask = c_uint::MAX;

        let dst_buffer_size = (dst_fmt.height * dst_fmt.width * 3) as usize;
        let layout = Layout::from_size_align(dst_buffer_size, 32).unwrap();
        let dst_buffer = unsafe { alloc(layout) };
        let mut dst : zimg_image_buffer = Default::default();
        dst.version = ZIMG_API_VERSION;
        dst.plane[0].data = dst_buffer as *mut c_void;
        dst.plane[0].stride = (dst_fmt.width) as isize;
        dst.plane[0].mask = c_uint::MAX;
        dst.plane[1].data = unsafe { dst_buffer.offset((dst_fmt.height * dst_fmt.width) as isize) } as *mut c_void;
        dst.plane[1].stride = (dst_fmt.width) as isize;
        dst.plane[1].mask = c_uint::MAX;
        dst.plane[2].data = unsafe { dst_buffer.offset((dst_fmt.height * dst_fmt.width * 2) as isize) } as *mut c_void;
        dst.plane[2].stride = (dst_fmt.width) as isize;
        dst.plane[2].mask = c_uint::MAX;

        let mut tmp_sz : usize = 0;
        let err = unsafe { zimg_filter_graph_get_tmp_size(graph, &mut tmp_sz) };
        assert_eq!(err, zimg_error_code_e::ZIMG_ERROR_SUCCESS);

        let layout = Layout::from_size_align(tmp_sz, 32).unwrap();
        let tmp = unsafe { alloc(layout) };

        let err = unsafe { zimg_filter_graph_process(graph, &src, &mut dst, tmp as *mut c_void, None, std::ptr::null_mut(), None, std::ptr::null_mut()) };
        assert_eq!(err, zimg_error_code_e::ZIMG_ERROR_SUCCESS);
    }
}