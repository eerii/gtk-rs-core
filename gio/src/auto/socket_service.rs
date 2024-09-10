// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, SocketConnection, SocketListener};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GSocketService")]
    pub struct SocketService(Object<ffi::GSocketService, ffi::GSocketServiceClass>) @extends SocketListener;

    match fn {
        type_ => || ffi::g_socket_service_get_type(),
    }
}

impl SocketService {
    pub const NONE: Option<&'static SocketService> = None;

    #[doc(alias = "g_socket_service_new")]
    pub fn new() -> SocketService {
        unsafe { from_glib_full(ffi::g_socket_service_new()) }
    }
}

impl Default for SocketService {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::SocketService>> Sealed for T {}
}

pub trait SocketServiceExt: IsA<SocketService> + sealed::Sealed + 'static {
    #[doc(alias = "g_socket_service_is_active")]
    #[doc(alias = "active")]
    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_service_is_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_service_start")]
    fn start(&self) {
        unsafe {
            ffi::g_socket_service_start(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "g_socket_service_stop")]
    fn stop(&self) {
        unsafe {
            ffi::g_socket_service_stop(self.as_ref().to_glib_none().0);
        }
    }

    fn set_active(&self, active: bool) {
        ObjectExt::set_property(self.as_ref(), "active", active)
    }

    #[doc(alias = "incoming")]
    fn connect_incoming<
        F: Fn(&Self, &SocketConnection, Option<&glib::Object>) -> bool + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn incoming_trampoline<
            P: IsA<SocketService>,
            F: Fn(&P, &SocketConnection, Option<&glib::Object>) -> bool + 'static,
        >(
            this: *mut ffi::GSocketService,
            connection: *mut ffi::GSocketConnection,
            source_object: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                SocketService::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(connection),
                Option::<glib::Object>::from_glib_borrow(source_object)
                    .as_ref()
                    .as_ref(),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"incoming\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    incoming_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "active")]
    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<
            P: IsA<SocketService>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GSocketService,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketService::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<SocketService>> SocketServiceExt for O {}
