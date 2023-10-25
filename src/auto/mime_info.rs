// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::{translate::*};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MimeInfo(Shared<ffi::WebKitMimeInfo>);

    match fn {
        ref => |ptr| ffi::webkit_mime_info_ref(ptr),
        unref => |ptr| ffi::webkit_mime_info_unref(ptr),
        type_ => || ffi::webkit_mime_info_get_type(),
    }
}

impl MimeInfo {
    #[cfg_attr(feature = "v2_32", deprecated = "Since 2.32")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_mime_info_get_description")]
    #[doc(alias = "get_description")]
    pub fn description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_mime_info_get_description(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v2_32", deprecated = "Since 2.32")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_mime_info_get_extensions")]
    #[doc(alias = "get_extensions")]
    pub fn extensions(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_mime_info_get_extensions(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v2_32", deprecated = "Since 2.32")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_mime_info_get_mime_type")]
    #[doc(alias = "get_mime_type")]
    pub fn mime_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_mime_info_get_mime_type(self.to_glib_none().0))
        }
    }
}
