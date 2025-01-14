// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};

glib::wrapper! {
	#[doc(alias = "WebKitUserMessage")]
	pub struct UserMessage(Object<ffi::WebKitUserMessage, ffi::WebKitUserMessageClass>);

	match fn {
		type_ => || ffi::webkit_user_message_get_type(),
	}
}

impl UserMessage {
	pub const NONE:Option<&'static UserMessage> = None;

	#[doc(alias = "webkit_user_message_new")]
	pub fn new(name:&str, parameters:Option<&glib::Variant>) -> UserMessage {
		assert_initialized_main_thread!();

		unsafe {
			from_glib_none(ffi::webkit_user_message_new(
				name.to_glib_none().0,
				parameters.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_user_message_new_with_fd_list")]
	#[doc(alias = "new_with_fd_list")]
	pub fn with_fd_list(
		name:&str,
		parameters:Option<&glib::Variant>,
		fd_list:Option<&impl IsA<gio::UnixFDList>>,
	) -> UserMessage {
		assert_initialized_main_thread!();

		unsafe {
			from_glib_none(ffi::webkit_user_message_new_with_fd_list(
				name.to_glib_none().0,
				parameters.to_glib_none().0,
				fd_list.map(|p| p.as_ref()).to_glib_none().0,
			))
		}
	}

	// rustdoc-stripper-ignore-next
	/// Creates a new builder-pattern struct instance to construct
	/// [`UserMessage`] objects.
	///
	/// This method returns an instance of
	/// [`UserMessageBuilder`](crate::builders::UserMessageBuilder) which can be
	/// used to create [`UserMessage`] objects.
	pub fn builder() -> UserMessageBuilder { UserMessageBuilder::new() }
}

#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
impl Default for UserMessage {
	fn default() -> Self { glib::object::Object::new::<Self>() }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`UserMessage`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct UserMessageBuilder {
	builder:glib::object::ObjectBuilder<'static, UserMessage>,
}

impl UserMessageBuilder {
	fn new() -> Self { Self { builder:glib::object::Object::builder() } }

	#[cfg(feature = "v2_28")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
	pub fn fd_list(self, fd_list:&impl IsA<gio::UnixFDList>) -> Self {
		Self { builder:self.builder.property("fd-list", fd_list.clone().upcast()) }
	}

	#[cfg(feature = "v2_28")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
	pub fn name(self, name:impl Into<glib::GString>) -> Self {
		Self { builder:self.builder.property("name", name.into()) }
	}

	#[cfg(feature = "v2_28")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
	pub fn parameters(self, parameters:&glib::Variant) -> Self {
		Self { builder:self.builder.property("parameters", parameters.clone()) }
	}

	// rustdoc-stripper-ignore-next
	/// Build the [`UserMessage`].
	#[must_use = "Building the object from the builder is usually expensive and is not expected to \
	              have side effects"]
	pub fn build(self) -> UserMessage { self.builder.build() }
}

mod sealed {
	pub trait Sealed {}

	impl<T:super::IsA<super::UserMessage>> Sealed for T {}
}

pub trait UserMessageExt: IsA<UserMessage> + sealed::Sealed + 'static {
	#[doc(alias = "webkit_user_message_get_fd_list")]
	#[doc(alias = "get_fd_list")]
	fn fd_list(&self) -> Option<gio::UnixFDList> {
		unsafe {
			from_glib_none(ffi::webkit_user_message_get_fd_list(self.as_ref().to_glib_none().0))
		}
	}

	#[doc(alias = "webkit_user_message_get_name")]
	#[doc(alias = "get_name")]
	fn name(&self) -> Option<glib::GString> {
		unsafe { from_glib_none(ffi::webkit_user_message_get_name(self.as_ref().to_glib_none().0)) }
	}

	#[doc(alias = "webkit_user_message_get_parameters")]
	#[doc(alias = "get_parameters")]
	fn parameters(&self) -> Option<glib::Variant> {
		unsafe {
			from_glib_none(ffi::webkit_user_message_get_parameters(self.as_ref().to_glib_none().0))
		}
	}

	#[doc(alias = "webkit_user_message_send_reply")]
	fn send_reply(&self, reply:&impl IsA<UserMessage>) {
		unsafe {
			ffi::webkit_user_message_send_reply(
				self.as_ref().to_glib_none().0,
				reply.as_ref().to_glib_none().0,
			);
		}
	}
}

impl<O:IsA<UserMessage>> UserMessageExt for O {}
