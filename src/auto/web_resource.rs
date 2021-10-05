// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/wusyong/gir-files)
// DO NOT EDIT

use crate::URIRequest;
use crate::URIResponse;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WebKitWebResource")]
    pub struct WebResource(Object<ffi::WebKitWebResource, ffi::WebKitWebResourceClass>);

    match fn {
        type_ => || ffi::webkit_web_resource_get_type(),
    }
}

pub const NONE_WEB_RESOURCE: Option<&WebResource> = None;

pub trait WebResourceExt: 'static {
    #[doc(alias = "webkit_web_resource_get_data")]
    #[doc(alias = "get_data")]
    fn data<P: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static>(&self, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P);

    
    fn data_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<u8>, glib::Error>> + 'static>>;

    #[doc(alias = "webkit_web_resource_get_response")]
    #[doc(alias = "get_response")]
    fn response(&self) -> Option<URIResponse>;

    #[doc(alias = "webkit_web_resource_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> Option<glib::GString>;

    #[doc(alias = "failed")]
    fn connect_failed<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "failed-with-tls-errors")]
    fn connect_failed_with_tls_errors<F: Fn(&Self, &gio::TlsCertificate, gio::TlsCertificateFlags) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "finished")]
    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "received-data")]
    fn connect_received_data<F: Fn(&Self, u64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "sent-request")]
    fn connect_sent_request<F: Fn(&Self, &URIRequest, &URIResponse) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "response")]
    fn connect_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "uri")]
    fn connect_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebResource>> WebResourceExt for O {
    fn data<P: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static>(&self, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P) {
        let user_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn data_trampoline<P: FnOnce(Result<Vec<u8>, glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let ret = ffi::webkit_web_resource_get_data_finish(_source_object as *mut _, res, length.as_mut_ptr(), &mut error);
            let result = if error.is_null() { Ok(FromGlibContainer::from_glib_full_num(ret, length.assume_init() as usize)) } else { Err(from_glib_full(error)) };
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = data_trampoline::<P>;
        unsafe {
            ffi::webkit_web_resource_get_data(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn data_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<u8>, glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.data(
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    fn response(&self) -> Option<URIResponse> {
        unsafe {
            from_glib_none(ffi::webkit_web_resource_get_response(self.as_ref().to_glib_none().0))
        }
    }

    fn uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_web_resource_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_failed<F: Fn(&Self, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn failed_trampoline<P: IsA<WebResource>, F: Fn(&P, &glib::Error) + 'static>(this: *mut ffi::WebKitWebResource, error: *mut glib::ffi::GError, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebResource::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(error))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"failed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(failed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn connect_failed_with_tls_errors<F: Fn(&Self, &gio::TlsCertificate, gio::TlsCertificateFlags) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn failed_with_tls_errors_trampoline<P: IsA<WebResource>, F: Fn(&P, &gio::TlsCertificate, gio::TlsCertificateFlags) + 'static>(this: *mut ffi::WebKitWebResource, certificate: *mut gio::ffi::GTlsCertificate, errors: gio::ffi::GTlsCertificateFlags, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebResource::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(certificate), from_glib(errors))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"failed-with-tls-errors\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(failed_with_tls_errors_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn finished_trampoline<P: IsA<WebResource>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitWebResource, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebResource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"finished\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(finished_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_received_data<F: Fn(&Self, u64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn received_data_trampoline<P: IsA<WebResource>, F: Fn(&P, u64) + 'static>(this: *mut ffi::WebKitWebResource, data_length: u64, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebResource::from_glib_borrow(this).unsafe_cast_ref(), data_length)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"received-data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(received_data_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_sent_request<F: Fn(&Self, &URIRequest, &URIResponse) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn sent_request_trampoline<P: IsA<WebResource>, F: Fn(&P, &URIRequest, &URIResponse) + 'static>(this: *mut ffi::WebKitWebResource, request: *mut ffi::WebKitURIRequest, redirected_response: *mut ffi::WebKitURIResponse, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebResource::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(request), &from_glib_borrow(redirected_response))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"sent-request\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(sent_request_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_response_trampoline<P: IsA<WebResource>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitWebResource, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebResource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::response\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_response_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<P: IsA<WebResource>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitWebResource, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(WebResource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_uri_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for WebResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebResource")
    }
}
