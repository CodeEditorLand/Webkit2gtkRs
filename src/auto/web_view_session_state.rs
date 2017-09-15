// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
#[cfg(feature = "v2_12")]
use glib;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct WebViewSessionState(Shared<ffi::WebKitWebViewSessionState>);

    match fn {
        ref => |ptr| ffi::webkit_web_view_session_state_ref(ptr),
        unref => |ptr| ffi::webkit_web_view_session_state_unref(ptr),
        get_type => || ffi::webkit_web_view_session_state_get_type(),
    }
}

impl WebViewSessionState {
    #[cfg(feature = "v2_12")]
    pub fn new(data: &glib::Bytes) -> WebViewSessionState {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_web_view_session_state_new(data.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_12")]
    pub fn serialize(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(ffi::webkit_web_view_session_state_serialize(self.to_glib_none().0))
        }
    }
}