// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Credentials, IOStream};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GDBusAuthObserver")]
    pub struct DBusAuthObserver(Object<ffi::GDBusAuthObserver>);

    match fn {
        type_ => || ffi::g_dbus_auth_observer_get_type(),
    }
}

impl DBusAuthObserver {
    #[doc(alias = "g_dbus_auth_observer_new")]
    pub fn new() -> DBusAuthObserver {
        unsafe { from_glib_full(ffi::g_dbus_auth_observer_new()) }
    }

    #[doc(alias = "g_dbus_auth_observer_allow_mechanism")]
    pub fn allow_mechanism(&self, mechanism: &str) -> bool {
        unsafe {
            from_glib(ffi::g_dbus_auth_observer_allow_mechanism(
                self.to_glib_none().0,
                mechanism.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_auth_observer_authorize_authenticated_peer")]
    pub fn authorize_authenticated_peer(
        &self,
        stream: &impl IsA<IOStream>,
        credentials: Option<&Credentials>,
    ) -> bool {
        unsafe {
            from_glib(ffi::g_dbus_auth_observer_authorize_authenticated_peer(
                self.to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                credentials.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "allow-mechanism")]
    pub fn connect_allow_mechanism<F: Fn(&Self, &str) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn allow_mechanism_trampoline<
            F: Fn(&DBusAuthObserver, &str) -> bool + 'static,
        >(
            this: *mut ffi::GDBusAuthObserver,
            mechanism: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(mechanism),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"allow-mechanism\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    allow_mechanism_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "authorize-authenticated-peer")]
    pub fn connect_authorize_authenticated_peer<
        F: Fn(&Self, &IOStream, Option<&Credentials>) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn authorize_authenticated_peer_trampoline<
            F: Fn(&DBusAuthObserver, &IOStream, Option<&Credentials>) -> bool + 'static,
        >(
            this: *mut ffi::GDBusAuthObserver,
            stream: *mut ffi::GIOStream,
            credentials: *mut ffi::GCredentials,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(stream),
                Option::<Credentials>::from_glib_borrow(credentials)
                    .as_ref()
                    .as_ref(),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"authorize-authenticated-peer\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    authorize_authenticated_peer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for DBusAuthObserver {
    fn default() -> Self {
        Self::new()
    }
}
