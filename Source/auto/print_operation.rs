// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
use crate::PrintCustomWidget;
use crate::{PrintOperationResponse, WebView};
use glib::{
	prelude::*,
	signal::{connect_raw, SignalHandlerId},
	translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
	#[doc(alias = "WebKitPrintOperation")]
	pub struct PrintOperation(Object<ffi::WebKitPrintOperation, ffi::WebKitPrintOperationClass>);

	match fn {
		type_ => || ffi::webkit_print_operation_get_type(),
	}
}

impl PrintOperation {
	pub const NONE: Option<&'static PrintOperation> = None;

	#[doc(alias = "webkit_print_operation_new")]
	pub fn new(web_view: &impl IsA<WebView>) -> PrintOperation {
		skip_assert_initialized!();
		unsafe {
			from_glib_full(ffi::webkit_print_operation_new(web_view.as_ref().to_glib_none().0))
		}
	}

	// rustdoc-stripper-ignore-next
	/// Creates a new builder-pattern struct instance to construct [`PrintOperation`] objects.
	///
	/// This method returns an instance of [`PrintOperationBuilder`](crate::builders::PrintOperationBuilder) which can be used to create [`PrintOperation`] objects.
	pub fn builder() -> PrintOperationBuilder {
		PrintOperationBuilder::new()
	}
}

impl Default for PrintOperation {
	fn default() -> Self {
		glib::object::Object::new::<Self>()
	}
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PrintOperation`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PrintOperationBuilder {
	builder: glib::object::ObjectBuilder<'static, PrintOperation>,
}

impl PrintOperationBuilder {
	fn new() -> Self {
		Self { builder: glib::object::Object::builder() }
	}

	pub fn page_setup(self, page_setup: &gtk::PageSetup) -> Self {
		Self { builder: self.builder.property("page-setup", page_setup.clone()) }
	}

	pub fn print_settings(self, print_settings: &gtk::PrintSettings) -> Self {
		Self { builder: self.builder.property("print-settings", print_settings.clone()) }
	}

	pub fn web_view(self, web_view: &impl IsA<WebView>) -> Self {
		Self { builder: self.builder.property("web-view", web_view.clone().upcast()) }
	}

	// rustdoc-stripper-ignore-next
	/// Build the [`PrintOperation`].
	#[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
	pub fn build(self) -> PrintOperation {
		self.builder.build()
	}
}

mod sealed {
	pub trait Sealed {}
	impl<T: super::IsA<super::PrintOperation>> Sealed for T {}
}

pub trait PrintOperationExt: IsA<PrintOperation> + sealed::Sealed + 'static {
	#[doc(alias = "webkit_print_operation_get_page_setup")]
	#[doc(alias = "get_page_setup")]
	fn page_setup(&self) -> Option<gtk::PageSetup> {
		unsafe {
			from_glib_none(ffi::webkit_print_operation_get_page_setup(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_print_operation_get_print_settings")]
	#[doc(alias = "get_print_settings")]
	fn print_settings(&self) -> Option<gtk::PrintSettings> {
		unsafe {
			from_glib_none(ffi::webkit_print_operation_get_print_settings(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_print_operation_print")]
	fn print(&self) {
		unsafe {
			ffi::webkit_print_operation_print(self.as_ref().to_glib_none().0);
		}
	}

	#[doc(alias = "webkit_print_operation_run_dialog")]
	fn run_dialog(&self, parent: Option<&impl IsA<gtk::Window>>) -> PrintOperationResponse {
		unsafe {
			from_glib(ffi::webkit_print_operation_run_dialog(
				self.as_ref().to_glib_none().0,
				parent.map(|p| p.as_ref()).to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_print_operation_set_page_setup")]
	fn set_page_setup(&self, page_setup: &gtk::PageSetup) {
		unsafe {
			ffi::webkit_print_operation_set_page_setup(
				self.as_ref().to_glib_none().0,
				page_setup.to_glib_none().0,
			);
		}
	}

	#[doc(alias = "webkit_print_operation_set_print_settings")]
	fn set_print_settings(&self, print_settings: &gtk::PrintSettings) {
		unsafe {
			ffi::webkit_print_operation_set_print_settings(
				self.as_ref().to_glib_none().0,
				print_settings.to_glib_none().0,
			);
		}
	}

	#[doc(alias = "web-view")]
	fn web_view(&self) -> Option<WebView> {
		ObjectExt::property(self.as_ref(), "web-view")
	}

	#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
	#[cfg(feature = "v2_16")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
	#[doc(alias = "create-custom-widget")]
	fn connect_create_custom_widget<F: Fn(&Self) -> PrintCustomWidget + 'static>(
		&self,
		f: F,
	) -> SignalHandlerId {
		unsafe extern fn create_custom_widget_trampoline<
			P: IsA<PrintOperation>,
			F: Fn(&P) -> PrintCustomWidget + 'static,
		>(
			this: *mut ffi::WebKitPrintOperation,
			f: glib::ffi::gpointer,
		) -> *mut ffi::WebKitPrintCustomWidget {
			let f: &F = &*(f as *const F);
			f(PrintOperation::from_glib_borrow(this).unsafe_cast_ref()).to_glib_full()
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"create-custom-widget\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					create_custom_widget_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[doc(alias = "failed")]
	fn connect_failed<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId {
		unsafe extern fn failed_trampoline<
			P: IsA<PrintOperation>,
			F: Fn(&P, &glib::Error) + 'static,
		>(
			this: *mut ffi::WebKitPrintOperation,
			error: *mut glib::ffi::GError,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(PrintOperation::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(error))
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"failed\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					failed_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[doc(alias = "finished")]
	fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
		unsafe extern fn finished_trampoline<P: IsA<PrintOperation>, F: Fn(&P) + 'static>(
			this: *mut ffi::WebKitPrintOperation,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(PrintOperation::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"finished\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					finished_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[doc(alias = "page-setup")]
	fn connect_page_setup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
		unsafe extern fn notify_page_setup_trampoline<
			P: IsA<PrintOperation>,
			F: Fn(&P) + 'static,
		>(
			this: *mut ffi::WebKitPrintOperation,
			_param_spec: glib::ffi::gpointer,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(PrintOperation::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::page-setup\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					notify_page_setup_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[doc(alias = "print-settings")]
	fn connect_print_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
		unsafe extern fn notify_print_settings_trampoline<
			P: IsA<PrintOperation>,
			F: Fn(&P) + 'static,
		>(
			this: *mut ffi::WebKitPrintOperation,
			_param_spec: glib::ffi::gpointer,
			f: glib::ffi::gpointer,
		) {
			let f: &F = &*(f as *const F);
			f(PrintOperation::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::print-settings\0".as_ptr() as *const _,
				Some(std::mem::transmute::<_, unsafe extern fn()>(
					notify_print_settings_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl<O: IsA<PrintOperation>> PrintOperationExt for O {}