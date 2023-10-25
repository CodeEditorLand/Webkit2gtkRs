// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use crate::{PermissionRequest};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "WebKitWebsiteDataAccessPermissionRequest")]
    pub struct WebsiteDataAccessPermissionRequest(Object<ffi::WebKitWebsiteDataAccessPermissionRequest, ffi::WebKitWebsiteDataAccessPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        type_ => || ffi::webkit_website_data_access_permission_request_get_type(),
    }
}

impl WebsiteDataAccessPermissionRequest {
        pub const NONE: Option<&'static WebsiteDataAccessPermissionRequest> = None;
    
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::WebsiteDataAccessPermissionRequest>> Sealed for T {}
}

pub trait WebsiteDataAccessPermissionRequestExt: IsA<WebsiteDataAccessPermissionRequest> + sealed::Sealed + 'static {
    #[doc(alias = "webkit_website_data_access_permission_request_get_current_domain")]
    #[doc(alias = "get_current_domain")]
    fn current_domain(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_access_permission_request_get_current_domain(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_website_data_access_permission_request_get_requesting_domain")]
    #[doc(alias = "get_requesting_domain")]
    fn requesting_domain(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_access_permission_request_get_requesting_domain(self.as_ref().to_glib_none().0))
        }
    }
}

impl<O: IsA<WebsiteDataAccessPermissionRequest>> WebsiteDataAccessPermissionRequestExt for O {}
