// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, TlsDatabase};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GTlsFileDatabase")]
    pub struct TlsFileDatabase(Interface<ffi::GTlsFileDatabase, ffi::GTlsFileDatabaseInterface>) @requires TlsDatabase;

    match fn {
        type_ => || ffi::g_tls_file_database_get_type(),
    }
}

impl TlsFileDatabase {
    pub const NONE: Option<&'static TlsFileDatabase> = None;

    #[doc(alias = "g_tls_file_database_new")]
    pub fn new(anchors: impl AsRef<std::path::Path>) -> Result<TlsFileDatabase, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_tls_file_database_new(anchors.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::TlsFileDatabase>> Sealed for T {}
}

pub trait TlsFileDatabaseExt: IsA<TlsFileDatabase> + sealed::Sealed + 'static {
    fn anchors(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "anchors")
    }

    fn set_anchors(&self, anchors: Option<&str>) {
        ObjectExt::set_property(self.as_ref(), "anchors", anchors)
    }

    #[doc(alias = "anchors")]
    fn connect_anchors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_anchors_trampoline<
            P: IsA<TlsFileDatabase>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsFileDatabase,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TlsFileDatabase::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::anchors\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_anchors_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<TlsFileDatabase>> TlsFileDatabaseExt for O {}
