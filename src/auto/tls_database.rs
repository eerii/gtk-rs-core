// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use SocketConnectable;
use TlsCertificate;
use TlsCertificateFlags;
use TlsDatabaseLookupFlags;
use TlsDatabaseVerifyFlags;
use TlsInteraction;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TlsDatabase(Object<ffi::GTlsDatabase, ffi::GTlsDatabaseClass>);

    match fn {
        get_type => || ffi::g_tls_database_get_type(),
    }
}

pub trait TlsDatabaseExt: Sized {
    fn create_certificate_handle(&self, certificate: &TlsCertificate) -> Option<String>;

    fn lookup_certificate_for_handle<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>>(&self, handle: &str, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q) -> Result<Option<TlsCertificate>, Error>;

    fn lookup_certificate_for_handle_async<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<TlsCertificate, Error>) + Send + 'static>(&self, handle: &str, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    fn lookup_certificate_for_handle_async_future<'a, P: Into<Option<&'a TlsInteraction>>>(&self, handle: &str, interaction: P, flags: TlsDatabaseLookupFlags) -> Box_<futures_core::Future<Item = (Self, TlsCertificate), Error = (Self, Error)>>;

    fn lookup_certificate_issuer<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>>(&self, certificate: &TlsCertificate, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q) -> Result<TlsCertificate, Error>;

    fn lookup_certificate_issuer_async<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<TlsCertificate, Error>) + Send + 'static>(&self, certificate: &TlsCertificate, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    fn lookup_certificate_issuer_async_future<'a, P: Into<Option<&'a TlsInteraction>>>(&self, certificate: &TlsCertificate, interaction: P, flags: TlsDatabaseLookupFlags) -> Box_<futures_core::Future<Item = (Self, TlsCertificate), Error = (Self, Error)>>;

    //fn lookup_certificates_issued_by<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q) -> Result<Vec<TlsCertificate>, Error>;

    //fn lookup_certificates_issued_by_async<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<Vec<TlsCertificate>, Error>) + Send + 'static>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q, callback: R);

    //#[cfg(feature = "futures")]
    //fn lookup_certificates_issued_by_async_future<'a, P: Into<Option<&'a TlsInteraction>>>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: P, flags: TlsDatabaseLookupFlags) -> Box_<futures_core::Future<Item = (Self, Vec<TlsCertificate>), Error = (Self, Error)>>;

    fn verify_chain<'a, 'b, 'c, P: IsA<SocketConnectable> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b TlsInteraction>>, S: Into<Option<&'c Cancellable>>>(&self, chain: &TlsCertificate, purpose: &str, identity: Q, interaction: R, flags: TlsDatabaseVerifyFlags, cancellable: S) -> Result<TlsCertificateFlags, Error>;

    fn verify_chain_async<'a, 'b, 'c, P: IsA<SocketConnectable> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b TlsInteraction>>, S: Into<Option<&'c Cancellable>>, T: FnOnce(Result<TlsCertificateFlags, Error>) + Send + 'static>(&self, chain: &TlsCertificate, purpose: &str, identity: Q, interaction: R, flags: TlsDatabaseVerifyFlags, cancellable: S, callback: T);

    #[cfg(feature = "futures")]
    fn verify_chain_async_future<'a, 'b, P: IsA<SocketConnectable> + Clone + 'static, Q: Into<Option<&'a P>>, R: Into<Option<&'b TlsInteraction>>>(&self, chain: &TlsCertificate, purpose: &str, identity: Q, interaction: R, flags: TlsDatabaseVerifyFlags) -> Box_<futures_core::Future<Item = (Self, TlsCertificateFlags), Error = (Self, Error)>>;
}

impl<O: IsA<TlsDatabase> + IsA<glib::object::Object> + Clone + 'static> TlsDatabaseExt for O {
    fn create_certificate_handle(&self, certificate: &TlsCertificate) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_tls_database_create_certificate_handle(self.to_glib_none().0, certificate.to_glib_none().0))
        }
    }

    fn lookup_certificate_for_handle<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>>(&self, handle: &str, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q) -> Result<Option<TlsCertificate>, Error> {
        let interaction = interaction.into();
        let interaction = interaction.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_database_lookup_certificate_for_handle(self.to_glib_none().0, handle.to_glib_none().0, interaction.0, flags.to_glib(), cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn lookup_certificate_for_handle_async<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<TlsCertificate, Error>) + Send + 'static>(&self, handle: &str, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q, callback: R) {
        let interaction = interaction.into();
        let interaction = interaction.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn lookup_certificate_for_handle_async_trampoline<R: FnOnce(Result<TlsCertificate, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_database_lookup_certificate_for_handle_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_certificate_for_handle_async_trampoline::<R>;
        unsafe {
            ffi::g_tls_database_lookup_certificate_for_handle_async(self.to_glib_none().0, handle.to_glib_none().0, interaction.0, flags.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn lookup_certificate_for_handle_async_future<'a, P: Into<Option<&'a TlsInteraction>>>(&self, handle: &str, interaction: P, flags: TlsDatabaseLookupFlags) -> Box_<futures_core::Future<Item = (Self, TlsCertificate), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        let handle = String::from(handle);
        let interaction = interaction.into();
        let interaction = interaction.cloned();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.lookup_certificate_for_handle_async(
                 &handle,
                 interaction.as_ref(),
                 flags,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn lookup_certificate_issuer<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>>(&self, certificate: &TlsCertificate, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q) -> Result<TlsCertificate, Error> {
        let interaction = interaction.into();
        let interaction = interaction.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_database_lookup_certificate_issuer(self.to_glib_none().0, certificate.to_glib_none().0, interaction.0, flags.to_glib(), cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn lookup_certificate_issuer_async<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<TlsCertificate, Error>) + Send + 'static>(&self, certificate: &TlsCertificate, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q, callback: R) {
        let interaction = interaction.into();
        let interaction = interaction.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn lookup_certificate_issuer_async_trampoline<R: FnOnce(Result<TlsCertificate, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_database_lookup_certificate_issuer_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_certificate_issuer_async_trampoline::<R>;
        unsafe {
            ffi::g_tls_database_lookup_certificate_issuer_async(self.to_glib_none().0, certificate.to_glib_none().0, interaction.0, flags.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn lookup_certificate_issuer_async_future<'a, P: Into<Option<&'a TlsInteraction>>>(&self, certificate: &TlsCertificate, interaction: P, flags: TlsDatabaseLookupFlags) -> Box_<futures_core::Future<Item = (Self, TlsCertificate), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        let certificate = certificate.clone();
        let interaction = interaction.into();
        let interaction = interaction.cloned();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.lookup_certificate_issuer_async(
                 &certificate,
                 interaction.as_ref(),
                 flags,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    //fn lookup_certificates_issued_by<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q) -> Result<Vec<TlsCertificate>, Error> {
    //    unsafe { TODO: call ffi::g_tls_database_lookup_certificates_issued_by() }
    //}

    //fn lookup_certificates_issued_by_async<'a, 'b, P: Into<Option<&'a TlsInteraction>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<Vec<TlsCertificate>, Error>) + Send + 'static>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: P, flags: TlsDatabaseLookupFlags, cancellable: Q, callback: R) {
    //    unsafe { TODO: call ffi::g_tls_database_lookup_certificates_issued_by_async() }
    //}

    //#[cfg(feature = "futures")]
    //fn lookup_certificates_issued_by_async_future<'a, P: Into<Option<&'a TlsInteraction>>>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: P, flags: TlsDatabaseLookupFlags) -> Box_<futures_core::Future<Item = (Self, Vec<TlsCertificate>), Error = (Self, Error)>> {
        //use GioFuture;
        //use send_cell::SendCell;

        //let issuer_raw_dn = issuer_raw_dn.clone();
        //let interaction = interaction.into();
        //let interaction = interaction.cloned();
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = Cancellable::new();
        //    let send = SendCell::new(send);
        //    let obj_clone = SendCell::new(obj.clone());
        //    obj.lookup_certificates_issued_by_async(
        //         &issuer_raw_dn,
        //         interaction.as_ref(),
        //         flags,
        //         Some(&cancellable),
        //         move |res| {
        //             let obj = obj_clone.into_inner();
        //             let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
        //             let _ = send.into_inner().send(res);
        //         },
        //    );

        //    cancellable
        //})
    //}

    fn verify_chain<'a, 'b, 'c, P: IsA<SocketConnectable> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b TlsInteraction>>, S: Into<Option<&'c Cancellable>>>(&self, chain: &TlsCertificate, purpose: &str, identity: Q, interaction: R, flags: TlsDatabaseVerifyFlags, cancellable: S) -> Result<TlsCertificateFlags, Error> {
        let identity = identity.into();
        let identity = identity.to_glib_none();
        let interaction = interaction.into();
        let interaction = interaction.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_database_verify_chain(self.to_glib_none().0, chain.to_glib_none().0, purpose.to_glib_none().0, identity.0, interaction.0, flags.to_glib(), cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn verify_chain_async<'a, 'b, 'c, P: IsA<SocketConnectable> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b TlsInteraction>>, S: Into<Option<&'c Cancellable>>, T: FnOnce(Result<TlsCertificateFlags, Error>) + Send + 'static>(&self, chain: &TlsCertificate, purpose: &str, identity: Q, interaction: R, flags: TlsDatabaseVerifyFlags, cancellable: S, callback: T) {
        let identity = identity.into();
        let identity = identity.to_glib_none();
        let interaction = interaction.into();
        let interaction = interaction.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<T>> = Box::new(Box::new(callback));
        unsafe extern "C" fn verify_chain_async_trampoline<T: FnOnce(Result<TlsCertificateFlags, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_database_verify_chain_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<T>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = verify_chain_async_trampoline::<T>;
        unsafe {
            ffi::g_tls_database_verify_chain_async(self.to_glib_none().0, chain.to_glib_none().0, purpose.to_glib_none().0, identity.0, interaction.0, flags.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn verify_chain_async_future<'a, 'b, P: IsA<SocketConnectable> + Clone + 'static, Q: Into<Option<&'a P>>, R: Into<Option<&'b TlsInteraction>>>(&self, chain: &TlsCertificate, purpose: &str, identity: Q, interaction: R, flags: TlsDatabaseVerifyFlags) -> Box_<futures_core::Future<Item = (Self, TlsCertificateFlags), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        let chain = chain.clone();
        let purpose = String::from(purpose);
        let identity = identity.into();
        let identity = identity.cloned();
        let interaction = interaction.into();
        let interaction = interaction.cloned();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.verify_chain_async(
                 &chain,
                 &purpose,
                 identity.as_ref(),
                 interaction.as_ref(),
                 flags,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }
}
