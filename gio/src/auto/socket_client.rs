// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, AsyncResult, Cancellable, IOStream, ProxyResolver, SocketAddress, SocketClientEvent,
    SocketConnectable, SocketConnection, SocketFamily, SocketProtocol, SocketType,
    TlsCertificateFlags,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GSocketClient")]
    pub struct SocketClient(Object<ffi::GSocketClient, ffi::GSocketClientClass>);

    match fn {
        type_ => || ffi::g_socket_client_get_type(),
    }
}

impl SocketClient {
    pub const NONE: Option<&'static SocketClient> = None;

    #[doc(alias = "g_socket_client_new")]
    pub fn new() -> SocketClient {
        unsafe { from_glib_full(ffi::g_socket_client_new()) }
    }
}

impl Default for SocketClient {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::SocketClient>> Sealed for T {}
}

pub trait SocketClientExt: IsA<SocketClient> + sealed::Sealed + 'static {
    #[doc(alias = "g_socket_client_add_application_proxy")]
    fn add_application_proxy(&self, protocol: &str) {
        unsafe {
            ffi::g_socket_client_add_application_proxy(
                self.as_ref().to_glib_none().0,
                protocol.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_socket_client_connect")]
    fn connect(
        &self,
        connectable: &impl IsA<SocketConnectable>,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<SocketConnection, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_socket_client_connect(
                self.as_ref().to_glib_none().0,
                connectable.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_socket_client_connect_async")]
    fn connect_async<P: FnOnce(Result<SocketConnection, glib::Error>) + 'static>(
        &self,
        connectable: &impl IsA<SocketConnectable>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn connect_async_trampoline<
            P: FnOnce(Result<SocketConnection, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::g_socket_client_connect_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = connect_async_trampoline::<P>;
        unsafe {
            ffi::g_socket_client_connect_async(
                self.as_ref().to_glib_none().0,
                connectable.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn connect_future(
        &self,
        connectable: &(impl IsA<SocketConnectable> + Clone + 'static),
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>
    {
        let connectable = connectable.clone();
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.connect_async(&connectable, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_socket_client_connect_to_host")]
    fn connect_to_host(
        &self,
        host_and_port: &str,
        default_port: u16,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<SocketConnection, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_host(
                self.as_ref().to_glib_none().0,
                host_and_port.to_glib_none().0,
                default_port,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_socket_client_connect_to_host_async")]
    fn connect_to_host_async<P: FnOnce(Result<SocketConnection, glib::Error>) + 'static>(
        &self,
        host_and_port: &str,
        default_port: u16,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn connect_to_host_async_trampoline<
            P: FnOnce(Result<SocketConnection, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_host_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = connect_to_host_async_trampoline::<P>;
        unsafe {
            ffi::g_socket_client_connect_to_host_async(
                self.as_ref().to_glib_none().0,
                host_and_port.to_glib_none().0,
                default_port,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn connect_to_host_future(
        &self,
        host_and_port: &str,
        default_port: u16,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>
    {
        let host_and_port = String::from(host_and_port);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.connect_to_host_async(
                    &host_and_port,
                    default_port,
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    #[doc(alias = "g_socket_client_connect_to_service")]
    fn connect_to_service(
        &self,
        domain: &str,
        service: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<SocketConnection, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_service(
                self.as_ref().to_glib_none().0,
                domain.to_glib_none().0,
                service.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_socket_client_connect_to_service_async")]
    fn connect_to_service_async<P: FnOnce(Result<SocketConnection, glib::Error>) + 'static>(
        &self,
        domain: &str,
        service: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn connect_to_service_async_trampoline<
            P: FnOnce(Result<SocketConnection, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_service_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = connect_to_service_async_trampoline::<P>;
        unsafe {
            ffi::g_socket_client_connect_to_service_async(
                self.as_ref().to_glib_none().0,
                domain.to_glib_none().0,
                service.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn connect_to_service_future(
        &self,
        domain: &str,
        service: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>
    {
        let domain = String::from(domain);
        let service = String::from(service);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.connect_to_service_async(&domain, &service, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_socket_client_connect_to_uri")]
    fn connect_to_uri(
        &self,
        uri: &str,
        default_port: u16,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<SocketConnection, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                default_port,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_socket_client_connect_to_uri_async")]
    fn connect_to_uri_async<P: FnOnce(Result<SocketConnection, glib::Error>) + 'static>(
        &self,
        uri: &str,
        default_port: u16,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn connect_to_uri_async_trampoline<
            P: FnOnce(Result<SocketConnection, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_uri_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = connect_to_uri_async_trampoline::<P>;
        unsafe {
            ffi::g_socket_client_connect_to_uri_async(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                default_port,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn connect_to_uri_future(
        &self,
        uri: &str,
        default_port: u16,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketConnection, glib::Error>> + 'static>>
    {
        let uri = String::from(uri);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.connect_to_uri_async(&uri, default_port, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_socket_client_get_enable_proxy")]
    #[doc(alias = "get_enable_proxy")]
    #[doc(alias = "enable-proxy")]
    fn enables_proxy(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_client_get_enable_proxy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_client_get_family")]
    #[doc(alias = "get_family")]
    fn family(&self) -> SocketFamily {
        unsafe {
            from_glib(ffi::g_socket_client_get_family(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_client_get_local_address")]
    #[doc(alias = "get_local_address")]
    #[doc(alias = "local-address")]
    fn local_address(&self) -> Option<SocketAddress> {
        unsafe {
            from_glib_none(ffi::g_socket_client_get_local_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_client_get_protocol")]
    #[doc(alias = "get_protocol")]
    fn protocol(&self) -> SocketProtocol {
        unsafe {
            from_glib(ffi::g_socket_client_get_protocol(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_client_get_proxy_resolver")]
    #[doc(alias = "get_proxy_resolver")]
    #[doc(alias = "proxy-resolver")]
    fn proxy_resolver(&self) -> ProxyResolver {
        unsafe {
            from_glib_none(ffi::g_socket_client_get_proxy_resolver(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_client_get_socket_type")]
    #[doc(alias = "get_socket_type")]
    fn socket_type(&self) -> SocketType {
        unsafe {
            from_glib(ffi::g_socket_client_get_socket_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_client_get_timeout")]
    #[doc(alias = "get_timeout")]
    fn timeout(&self) -> u32 {
        unsafe { ffi::g_socket_client_get_timeout(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_socket_client_get_tls")]
    #[doc(alias = "get_tls")]
    #[doc(alias = "tls")]
    fn is_tls(&self) -> bool {
        unsafe { from_glib(ffi::g_socket_client_get_tls(self.as_ref().to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v2_72", deprecated = "Since 2.72")]
    #[allow(deprecated)]
    #[doc(alias = "g_socket_client_get_tls_validation_flags")]
    #[doc(alias = "get_tls_validation_flags")]
    #[doc(alias = "tls-validation-flags")]
    fn tls_validation_flags(&self) -> TlsCertificateFlags {
        unsafe {
            from_glib(ffi::g_socket_client_get_tls_validation_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_client_set_enable_proxy")]
    #[doc(alias = "enable-proxy")]
    fn set_enable_proxy(&self, enable: bool) {
        unsafe {
            ffi::g_socket_client_set_enable_proxy(
                self.as_ref().to_glib_none().0,
                enable.into_glib(),
            );
        }
    }

    #[doc(alias = "g_socket_client_set_family")]
    #[doc(alias = "family")]
    fn set_family(&self, family: SocketFamily) {
        unsafe {
            ffi::g_socket_client_set_family(self.as_ref().to_glib_none().0, family.into_glib());
        }
    }

    #[doc(alias = "g_socket_client_set_local_address")]
    #[doc(alias = "local-address")]
    fn set_local_address(&self, address: Option<&impl IsA<SocketAddress>>) {
        unsafe {
            ffi::g_socket_client_set_local_address(
                self.as_ref().to_glib_none().0,
                address.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_socket_client_set_protocol")]
    #[doc(alias = "protocol")]
    fn set_protocol(&self, protocol: SocketProtocol) {
        unsafe {
            ffi::g_socket_client_set_protocol(self.as_ref().to_glib_none().0, protocol.into_glib());
        }
    }

    #[doc(alias = "g_socket_client_set_proxy_resolver")]
    #[doc(alias = "proxy-resolver")]
    fn set_proxy_resolver(&self, proxy_resolver: Option<&impl IsA<ProxyResolver>>) {
        unsafe {
            ffi::g_socket_client_set_proxy_resolver(
                self.as_ref().to_glib_none().0,
                proxy_resolver.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_socket_client_set_socket_type")]
    fn set_socket_type(&self, type_: SocketType) {
        unsafe {
            ffi::g_socket_client_set_socket_type(self.as_ref().to_glib_none().0, type_.into_glib());
        }
    }

    #[doc(alias = "g_socket_client_set_timeout")]
    #[doc(alias = "timeout")]
    fn set_timeout(&self, timeout: u32) {
        unsafe {
            ffi::g_socket_client_set_timeout(self.as_ref().to_glib_none().0, timeout);
        }
    }

    #[doc(alias = "g_socket_client_set_tls")]
    #[doc(alias = "tls")]
    fn set_tls(&self, tls: bool) {
        unsafe {
            ffi::g_socket_client_set_tls(self.as_ref().to_glib_none().0, tls.into_glib());
        }
    }

    #[cfg_attr(feature = "v2_72", deprecated = "Since 2.72")]
    #[allow(deprecated)]
    #[doc(alias = "g_socket_client_set_tls_validation_flags")]
    #[doc(alias = "tls-validation-flags")]
    fn set_tls_validation_flags(&self, flags: TlsCertificateFlags) {
        unsafe {
            ffi::g_socket_client_set_tls_validation_flags(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
            );
        }
    }

    #[doc(alias = "type")]
    fn type_(&self) -> SocketType {
        ObjectExt::property(self.as_ref(), "type")
    }

    #[doc(alias = "type")]
    fn set_type(&self, type_: SocketType) {
        ObjectExt::set_property(self.as_ref(), "type", type_)
    }

    #[doc(alias = "event")]
    fn connect_event<
        F: Fn(&Self, SocketClientEvent, &SocketConnectable, Option<&IOStream>) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<
            P: IsA<SocketClient>,
            F: Fn(&P, SocketClientEvent, &SocketConnectable, Option<&IOStream>) + 'static,
        >(
            this: *mut ffi::GSocketClient,
            event: ffi::GSocketClientEvent,
            connectable: *mut ffi::GSocketConnectable,
            connection: *mut ffi::GIOStream,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                SocketClient::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(event),
                &from_glib_borrow(connectable),
                Option::<IOStream>::from_glib_borrow(connection)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"event\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "enable-proxy")]
    fn connect_enable_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_proxy_trampoline<
            P: IsA<SocketClient>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-proxy\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_enable_proxy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "family")]
    fn connect_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_family_trampoline<P: IsA<SocketClient>, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::family\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_family_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "local-address")]
    fn connect_local_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_address_trampoline<
            P: IsA<SocketClient>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local-address\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_local_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "protocol")]
    fn connect_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_protocol_trampoline<
            P: IsA<SocketClient>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::protocol\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_protocol_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "proxy-resolver")]
    fn connect_proxy_resolver_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_proxy_resolver_trampoline<
            P: IsA<SocketClient>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::proxy-resolver\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_proxy_resolver_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "timeout")]
    fn connect_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<
            P: IsA<SocketClient>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tls")]
    fn connect_tls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tls_trampoline<P: IsA<SocketClient>, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tls\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_tls_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v2_72", deprecated = "Since 2.72")]
    #[doc(alias = "tls-validation-flags")]
    fn connect_tls_validation_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tls_validation_flags_trampoline<
            P: IsA<SocketClient>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tls-validation-flags\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_tls_validation_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "type")]
    fn connect_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P: IsA<SocketClient>, F: Fn(&P) + 'static>(
            this: *mut ffi::GSocketClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::type\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<SocketClient>> SocketClientExt for O {}
