// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use glib::{prelude::*, translate::*};

use crate::{ContextMenu, ContextMenuAction};

glib::wrapper! {
	#[doc(alias = "WebKitContextMenuItem")]
	pub struct ContextMenuItem(Object<ffi::WebKitContextMenuItem, ffi::WebKitContextMenuItemClass>);

	match fn {
		type_ => || ffi::webkit_context_menu_item_get_type(),
	}
}

impl ContextMenuItem {
	pub const NONE:Option<&'static ContextMenuItem> = None;

	//#[cfg_attr(feature = "v2_18", deprecated = "Since 2.18")]
	//#[allow(deprecated)]
	//#[doc(alias = "webkit_context_menu_item_new")]
	// pub fn new(action: /*Ignored*/&gtk::Action) -> ContextMenuItem {
	//    unsafe { TODO: call ffi:webkit_context_menu_item_new() }
	//}

	#[cfg(feature = "v2_18")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
	#[doc(alias = "webkit_context_menu_item_new_from_gaction")]
	#[doc(alias = "new_from_gaction")]
	pub fn from_gaction(
		action:&impl IsA<gio::Action>,
		label:&str,
		target:Option<&glib::Variant>,
	) -> ContextMenuItem {
		assert_initialized_main_thread!();
		unsafe {
			from_glib_none(ffi::webkit_context_menu_item_new_from_gaction(
				action.as_ref().to_glib_none().0,
				label.to_glib_none().0,
				target.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_context_menu_item_new_from_stock_action")]
	#[doc(alias = "new_from_stock_action")]
	pub fn from_stock_action(action:ContextMenuAction) -> ContextMenuItem {
		assert_initialized_main_thread!();
		unsafe {
			from_glib_none(ffi::webkit_context_menu_item_new_from_stock_action(
				action.into_glib(),
			))
		}
	}

	#[doc(alias = "webkit_context_menu_item_new_from_stock_action_with_label")]
	#[doc(alias = "new_from_stock_action_with_label")]
	pub fn from_stock_action_with_label(
		action:ContextMenuAction,
		label:&str,
	) -> ContextMenuItem {
		assert_initialized_main_thread!();
		unsafe {
			from_glib_none(
				ffi::webkit_context_menu_item_new_from_stock_action_with_label(
					action.into_glib(),
					label.to_glib_none().0,
				),
			)
		}
	}

	#[doc(alias = "webkit_context_menu_item_new_separator")]
	pub fn new_separator() -> ContextMenuItem {
		assert_initialized_main_thread!();
		unsafe { from_glib_none(ffi::webkit_context_menu_item_new_separator()) }
	}

	#[doc(alias = "webkit_context_menu_item_new_with_submenu")]
	#[doc(alias = "new_with_submenu")]
	pub fn with_submenu(
		label:&str,
		submenu:&impl IsA<ContextMenu>,
	) -> ContextMenuItem {
		skip_assert_initialized!();
		unsafe {
			from_glib_none(ffi::webkit_context_menu_item_new_with_submenu(
				label.to_glib_none().0,
				submenu.as_ref().to_glib_none().0,
			))
		}
	}
}

mod sealed {
	pub trait Sealed {}
	impl<T:super::IsA<super::ContextMenuItem>> Sealed for T {}
}

pub trait ContextMenuItemExt:
	IsA<ContextMenuItem> + sealed::Sealed + 'static {
	//#[cfg_attr(feature = "v2_18", deprecated = "Since 2.18")]
	//#[allow(deprecated)]
	//#[doc(alias = "webkit_context_menu_item_get_action")]
	//#[doc(alias = "get_action")]
	// fn action(&self) -> /*Ignored*/Option<gtk::Action> {
	//    unsafe { TODO: call ffi:webkit_context_menu_item_get_action() }
	//}

	#[cfg(feature = "v2_18")]
	#[cfg_attr(docsrs, doc(cfg(feature = "v2_18")))]
	#[doc(alias = "webkit_context_menu_item_get_gaction")]
	#[doc(alias = "get_gaction")]
	fn gaction(&self) -> Option<gio::Action> {
		unsafe {
			from_glib_none(ffi::webkit_context_menu_item_get_gaction(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_context_menu_item_get_stock_action")]
	#[doc(alias = "get_stock_action")]
	fn stock_action(&self) -> ContextMenuAction {
		unsafe {
			from_glib(ffi::webkit_context_menu_item_get_stock_action(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_context_menu_item_get_submenu")]
	#[doc(alias = "get_submenu")]
	fn submenu(&self) -> Option<ContextMenu> {
		unsafe {
			from_glib_none(ffi::webkit_context_menu_item_get_submenu(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_context_menu_item_is_separator")]
	fn is_separator(&self) -> bool {
		unsafe {
			from_glib(ffi::webkit_context_menu_item_is_separator(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[doc(alias = "webkit_context_menu_item_set_submenu")]
	fn set_submenu(&self, submenu:Option<&impl IsA<ContextMenu>>) {
		unsafe {
			ffi::webkit_context_menu_item_set_submenu(
				self.as_ref().to_glib_none().0,
				submenu.map(|p| p.as_ref()).to_glib_none().0,
			);
		}
	}
}

impl<O:IsA<ContextMenuItem>> ContextMenuItemExt for O {}
