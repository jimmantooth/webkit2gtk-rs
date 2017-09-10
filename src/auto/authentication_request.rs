// This file was generated by gir (e95e5d8) from gir-files (857b8f5)
// DO NOT EDIT

#[cfg(feature = "v2_2")]
use AuthenticationScheme;
use ffi;
use glib;
#[cfg(feature = "v2_2")]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v2_2")]
use glib::signal::SignalHandlerId;
#[cfg(feature = "v2_2")]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v2_2")]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(feature = "v2_2")]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct AuthenticationRequest(Object<ffi::WebKitAuthenticationRequest>);

    match fn {
        get_type => || ffi::webkit_authentication_request_get_type(),
    }
}

pub trait AuthenticationRequestExt {
    //#[cfg(feature = "v2_2")]
    //fn authenticate<'a, P: Into<Option<&'a /*Ignored*/Credential>>>(&self, credential: P);

    #[cfg(feature = "v2_2")]
    fn can_save_credentials(&self) -> bool;

    #[cfg(feature = "v2_2")]
    fn cancel(&self);

    #[cfg(feature = "v2_2")]
    fn get_host(&self) -> Option<String>;

    #[cfg(feature = "v2_2")]
    fn get_port(&self) -> u32;

    //#[cfg(feature = "v2_2")]
    //fn get_proposed_credential(&self) -> /*Ignored*/Option<Credential>;

    #[cfg(feature = "v2_2")]
    fn get_realm(&self) -> Option<String>;

    #[cfg(feature = "v2_2")]
    fn get_scheme(&self) -> AuthenticationScheme;

    #[cfg(feature = "v2_2")]
    fn is_for_proxy(&self) -> bool;

    #[cfg(feature = "v2_2")]
    fn is_retry(&self) -> bool;

    #[cfg(feature = "v2_2")]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AuthenticationRequest> + IsA<glib::object::Object>> AuthenticationRequestExt for O {
    //#[cfg(feature = "v2_2")]
    //fn authenticate<'a, P: Into<Option<&'a /*Ignored*/Credential>>>(&self, credential: P) {
    //    unsafe { TODO: call ffi::webkit_authentication_request_authenticate() }
    //}

    #[cfg(feature = "v2_2")]
    fn can_save_credentials(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_can_save_credentials(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    fn cancel(&self) {
        unsafe {
            ffi::webkit_authentication_request_cancel(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_2")]
    fn get_host(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_host(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    fn get_port(&self) -> u32 {
        unsafe {
            ffi::webkit_authentication_request_get_port(self.to_glib_none().0)
        }
    }

    //#[cfg(feature = "v2_2")]
    //fn get_proposed_credential(&self) -> /*Ignored*/Option<Credential> {
    //    unsafe { TODO: call ffi::webkit_authentication_request_get_proposed_credential() }
    //}

    #[cfg(feature = "v2_2")]
    fn get_realm(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_realm(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    fn get_scheme(&self) -> AuthenticationScheme {
        unsafe {
            from_glib(ffi::webkit_authentication_request_get_scheme(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    fn is_for_proxy(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_for_proxy(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    fn is_retry(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_retry(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_2")]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancelled",
                transmute(cancelled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_2")]
unsafe extern "C" fn cancelled_trampoline<P>(this: *mut ffi::WebKitAuthenticationRequest, f: glib_ffi::gpointer)
where P: IsA<AuthenticationRequest> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AuthenticationRequest::from_glib_borrow(this).downcast_unchecked())
}
