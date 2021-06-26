/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#[cfg(feature = "v2_6")]
use std::ffi::CString;
#[cfg(feature = "v2_6")]
use std::ptr;

use glib::IsA;
#[cfg(feature = "v2_6")]
use glib::object::Cast;
#[cfg(feature = "v2_6")]
use glib::StaticType;
#[cfg(feature = "v2_6")]
use glib::translate::{FromGlibPtrNone, IntoGlib, ToGlibPtr};
#[cfg(feature = "v2_6")]
use gobject_sys;
use gtk;

#[cfg(feature = "v2_6")]
use super::WebContext;
use super::WebView;
#[cfg(feature = "v2_6")]
use super::UserContentManager;

pub trait WebViewExtManual {
    #[cfg(feature = "v2_6")]
    fn new_with_context_and_user_content_manager(context: &WebContext, user_content_manager: &UserContentManager) -> Self;
}

impl<O> WebViewExtManual for O
    where O: IsA<gtk::Widget> + IsA<WebView>
{
    #[cfg(feature = "v2_6")]
    fn new_with_context_and_user_content_manager(context: &WebContext, user_content_manager: &UserContentManager) -> Self {
        assert_initialized_main_thread!();
        let user_content_manager_property = CString::new("user-content-manager").unwrap();
        let web_context_property = CString::new("web-context").unwrap();
        let glib_user_content_manager: *mut webkit2_sys::WebKitUserContentManager = user_content_manager.to_glib_none().0;
        let glib_user_content_manager = glib_user_content_manager as *mut gobject_sys::GObject;
        let glib_context: *mut webkit2_sys::WebKitWebContext = context.to_glib_none().0;
        let glib_context = glib_context as *mut gobject_sys::GObject;
        let null: *mut gobject_sys::GObject = ptr::null_mut();
        unsafe {
            gtk::Widget::from_glib_none(gobject_sys::g_object_new(WebView::static_type().into_glib(), user_content_manager_property.as_ptr(), glib_user_content_manager, web_context_property.as_ptr(), glib_context, null) as *mut _).downcast().expect("downcast")
        }
    }
}
