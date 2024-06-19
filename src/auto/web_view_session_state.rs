// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct WebViewSessionState(Shared<ffi::WebKitWebViewSessionState>);

    match fn {
        ref => |ptr| ffi::webkit_web_view_session_state_ref(ptr),
        unref => |ptr| ffi::webkit_web_view_session_state_unref(ptr),
        type_ => || ffi::webkit_web_view_session_state_get_type(),
    }
}

impl WebViewSessionState {
  #[doc(alias = "webkit_web_view_session_state_new")]
  pub fn new(data: &glib::Bytes) -> WebViewSessionState {
    assert_initialized_main_thread!();
    unsafe {
      from_glib_full(ffi::webkit_web_view_session_state_new(
        data.to_glib_none().0,
      ))
    }
  }

  #[doc(alias = "webkit_web_view_session_state_serialize")]
  pub fn serialize(&self) -> Option<glib::Bytes> {
    unsafe {
      from_glib_full(ffi::webkit_web_view_session_state_serialize(
        self.to_glib_none().0,
      ))
    }
  }
}
