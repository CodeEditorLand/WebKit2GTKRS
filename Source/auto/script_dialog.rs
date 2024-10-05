// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::translate::*;

use crate::ScriptDialogType;

glib::wrapper! {
	#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct ScriptDialog(Shared<ffi::WebKitScriptDialog>);

	match fn {
		ref => |ptr| ffi::webkit_script_dialog_ref(ptr),
		unref => |ptr| ffi::webkit_script_dialog_unref(ptr),
		type_ => || ffi::webkit_script_dialog_get_type(),
	}
}

impl ScriptDialog {
	#[doc(alias = "webkit_script_dialog_close")]
	pub fn close(&self) {
		unsafe {
			ffi::webkit_script_dialog_close(self.to_glib_none().0);
		}
	}

	#[doc(alias = "webkit_script_dialog_confirm_set_confirmed")]
	pub fn confirm_set_confirmed(&self, confirmed:bool) {
		unsafe {
			ffi::webkit_script_dialog_confirm_set_confirmed(
				self.to_glib_none().0,
				confirmed.into_glib(),
			);
		}
	}

	#[doc(alias = "webkit_script_dialog_get_dialog_type")]
	#[doc(alias = "get_dialog_type")]
	pub fn dialog_type(&self) -> ScriptDialogType {
		unsafe {
			from_glib(ffi::webkit_script_dialog_get_dialog_type(
				self.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_script_dialog_get_message")]
	#[doc(alias = "get_message")]
	pub fn message(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::webkit_script_dialog_get_message(
				self.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_script_dialog_prompt_get_default_text")]
	pub fn prompt_get_default_text(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::webkit_script_dialog_prompt_get_default_text(
				self.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_script_dialog_prompt_set_text")]
	pub fn prompt_set_text(&self, text:&str) {
		unsafe {
			ffi::webkit_script_dialog_prompt_set_text(
				self.to_glib_none().0,
				text.to_glib_none().0,
			);
		}
	}
}
