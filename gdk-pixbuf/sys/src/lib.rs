// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use gio_sys as gio;
use glib_sys as glib;
use gobject_sys as gobject;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, off_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};
#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GdkColorspace = c_int;
pub const GDK_COLORSPACE_RGB: GdkColorspace = 0;

pub type GdkInterpType = c_int;
pub const GDK_INTERP_NEAREST: GdkInterpType = 0;
pub const GDK_INTERP_TILES: GdkInterpType = 1;
pub const GDK_INTERP_BILINEAR: GdkInterpType = 2;
pub const GDK_INTERP_HYPER: GdkInterpType = 3;

pub type GdkPixbufAlphaMode = c_int;
pub const GDK_PIXBUF_ALPHA_BILEVEL: GdkPixbufAlphaMode = 0;
pub const GDK_PIXBUF_ALPHA_FULL: GdkPixbufAlphaMode = 1;

pub type GdkPixbufError = c_int;
pub const GDK_PIXBUF_ERROR_CORRUPT_IMAGE: GdkPixbufError = 0;
pub const GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY: GdkPixbufError = 1;
pub const GDK_PIXBUF_ERROR_BAD_OPTION: GdkPixbufError = 2;
pub const GDK_PIXBUF_ERROR_UNKNOWN_TYPE: GdkPixbufError = 3;
pub const GDK_PIXBUF_ERROR_UNSUPPORTED_OPERATION: GdkPixbufError = 4;
pub const GDK_PIXBUF_ERROR_FAILED: GdkPixbufError = 5;
pub const GDK_PIXBUF_ERROR_INCOMPLETE_ANIMATION: GdkPixbufError = 6;

pub type GdkPixbufRotation = c_int;
pub const GDK_PIXBUF_ROTATE_NONE: GdkPixbufRotation = 0;
pub const GDK_PIXBUF_ROTATE_COUNTERCLOCKWISE: GdkPixbufRotation = 90;
pub const GDK_PIXBUF_ROTATE_UPSIDEDOWN: GdkPixbufRotation = 180;
pub const GDK_PIXBUF_ROTATE_CLOCKWISE: GdkPixbufRotation = 270;

// Constants

// Flags
pub type GdkPixbufFormatFlags = c_uint;
pub const GDK_PIXBUF_FORMAT_WRITABLE: GdkPixbufFormatFlags = 1;
pub const GDK_PIXBUF_FORMAT_SCALABLE: GdkPixbufFormatFlags = 2;
pub const GDK_PIXBUF_FORMAT_THREADSAFE: GdkPixbufFormatFlags = 4;

// Callbacks
pub type GdkPixbufDestroyNotify = Option<unsafe extern "C" fn(*mut u8, gpointer)>;
pub type GdkPixbufModuleBeginLoadFunc = Option<
    unsafe extern "C" fn(
        GdkPixbufModuleSizeFunc,
        GdkPixbufModulePreparedFunc,
        GdkPixbufModuleUpdatedFunc,
        gpointer,
        *mut *mut glib::GError,
    ) -> gpointer,
>;
pub type GdkPixbufModuleFillInfoFunc = Option<unsafe extern "C" fn(*mut GdkPixbufFormat)>;
pub type GdkPixbufModuleFillVtableFunc = Option<unsafe extern "C" fn(*mut GdkPixbufModule)>;
pub type GdkPixbufModuleIncrementLoadFunc =
    Option<unsafe extern "C" fn(gpointer, *const u8, c_uint, *mut *mut glib::GError) -> gboolean>;
pub type GdkPixbufModuleLoadAnimationFunc =
    Option<unsafe extern "C" fn(*mut FILE, *mut *mut glib::GError) -> *mut GdkPixbufAnimation>;
pub type GdkPixbufModuleLoadFunc =
    Option<unsafe extern "C" fn(*mut FILE, *mut *mut glib::GError) -> *mut GdkPixbuf>;
pub type GdkPixbufModuleLoadXpmDataFunc =
    Option<unsafe extern "C" fn(*mut *const c_char) -> *mut GdkPixbuf>;
pub type GdkPixbufModulePreparedFunc =
    Option<unsafe extern "C" fn(*mut GdkPixbuf, *mut GdkPixbufAnimation, gpointer)>;
pub type GdkPixbufModuleSaveCallbackFunc = Option<
    unsafe extern "C" fn(
        GdkPixbufSaveFunc,
        gpointer,
        *mut GdkPixbuf,
        *mut *mut c_char,
        *mut *mut c_char,
        *mut *mut glib::GError,
    ) -> gboolean,
>;
pub type GdkPixbufModuleSaveFunc = Option<
    unsafe extern "C" fn(
        *mut FILE,
        *mut GdkPixbuf,
        *mut *mut c_char,
        *mut *mut c_char,
        *mut *mut glib::GError,
    ) -> gboolean,
>;
pub type GdkPixbufModuleSaveOptionSupportedFunc =
    Option<unsafe extern "C" fn(*const c_char) -> gboolean>;
pub type GdkPixbufModuleSizeFunc = Option<unsafe extern "C" fn(*mut c_int, *mut c_int, gpointer)>;
pub type GdkPixbufModuleStopLoadFunc =
    Option<unsafe extern "C" fn(gpointer, *mut *mut glib::GError) -> gboolean>;
pub type GdkPixbufModuleUpdatedFunc =
    Option<unsafe extern "C" fn(*mut GdkPixbuf, c_int, c_int, c_int, c_int, gpointer)>;
pub type GdkPixbufSaveFunc =
    Option<unsafe extern "C" fn(*const u8, size_t, *mut *mut glib::GError, gpointer) -> gboolean>;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GdkPixbufAnimationClass {
    pub parent_class: gobject::GObjectClass,
    pub is_static_image: Option<unsafe extern "C" fn(*mut GdkPixbufAnimation) -> gboolean>,
    pub get_static_image: Option<unsafe extern "C" fn(*mut GdkPixbufAnimation) -> *mut GdkPixbuf>,
    pub get_size: Option<unsafe extern "C" fn(*mut GdkPixbufAnimation, *mut c_int, *mut c_int)>,
    pub get_iter: Option<
        unsafe extern "C" fn(
            *mut GdkPixbufAnimation,
            *const glib::GTimeVal,
        ) -> *mut GdkPixbufAnimationIter,
    >,
}

impl ::std::fmt::Debug for GdkPixbufAnimationClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufAnimationClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .field("is_static_image", &self.is_static_image)
            .field("get_static_image", &self.get_static_image)
            .field("get_size", &self.get_size)
            .field("get_iter", &self.get_iter)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GdkPixbufAnimationIterClass {
    pub parent_class: gobject::GObjectClass,
    pub get_delay_time: Option<unsafe extern "C" fn(*mut GdkPixbufAnimationIter) -> c_int>,
    pub get_pixbuf: Option<unsafe extern "C" fn(*mut GdkPixbufAnimationIter) -> *mut GdkPixbuf>,
    pub on_currently_loading_frame:
        Option<unsafe extern "C" fn(*mut GdkPixbufAnimationIter) -> gboolean>,
    pub advance: Option<
        unsafe extern "C" fn(*mut GdkPixbufAnimationIter, *const glib::GTimeVal) -> gboolean,
    >,
}

impl ::std::fmt::Debug for GdkPixbufAnimationIterClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufAnimationIterClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .field("get_delay_time", &self.get_delay_time)
            .field("get_pixbuf", &self.get_pixbuf)
            .field(
                "on_currently_loading_frame",
                &self.on_currently_loading_frame,
            )
            .field("advance", &self.advance)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GdkPixbufFormat {
    pub name: *mut c_char,
    pub signature: *mut GdkPixbufModulePattern,
    pub domain: *mut c_char,
    pub description: *mut c_char,
    pub mime_types: *mut *mut c_char,
    pub extensions: *mut *mut c_char,
    pub flags: u32,
    pub disabled: gboolean,
    pub license: *mut c_char,
}

impl ::std::fmt::Debug for GdkPixbufFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufFormat @ {self:p}"))
            .field("name", &self.name)
            .field("signature", &self.signature)
            .field("domain", &self.domain)
            .field("description", &self.description)
            .field("mime_types", &self.mime_types)
            .field("extensions", &self.extensions)
            .field("flags", &self.flags)
            .field("disabled", &self.disabled)
            .field("license", &self.license)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GdkPixbufLoaderClass {
    pub parent_class: gobject::GObjectClass,
    pub size_prepared: Option<unsafe extern "C" fn(*mut GdkPixbufLoader, c_int, c_int)>,
    pub area_prepared: Option<unsafe extern "C" fn(*mut GdkPixbufLoader)>,
    pub area_updated:
        Option<unsafe extern "C" fn(*mut GdkPixbufLoader, c_int, c_int, c_int, c_int)>,
    pub closed: Option<unsafe extern "C" fn(*mut GdkPixbufLoader)>,
}

impl ::std::fmt::Debug for GdkPixbufLoaderClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufLoaderClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .field("size_prepared", &self.size_prepared)
            .field("area_prepared", &self.area_prepared)
            .field("area_updated", &self.area_updated)
            .field("closed", &self.closed)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GdkPixbufModule {
    pub module_name: *mut c_char,
    pub module_path: *mut c_char,
    pub module: gpointer,
    pub info: *mut GdkPixbufFormat,
    pub load: GdkPixbufModuleLoadFunc,
    pub load_xpm_data: GdkPixbufModuleLoadXpmDataFunc,
    pub begin_load: GdkPixbufModuleBeginLoadFunc,
    pub stop_load: GdkPixbufModuleStopLoadFunc,
    pub load_increment: GdkPixbufModuleIncrementLoadFunc,
    pub load_animation: GdkPixbufModuleLoadAnimationFunc,
    pub save: GdkPixbufModuleSaveFunc,
    pub save_to_callback: GdkPixbufModuleSaveCallbackFunc,
    pub is_save_option_supported: GdkPixbufModuleSaveOptionSupportedFunc,
    pub _reserved1: Option<unsafe extern "C" fn()>,
    pub _reserved2: Option<unsafe extern "C" fn()>,
    pub _reserved3: Option<unsafe extern "C" fn()>,
    pub _reserved4: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for GdkPixbufModule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufModule @ {self:p}"))
            .field("module_name", &self.module_name)
            .field("module_path", &self.module_path)
            .field("module", &self.module)
            .field("info", &self.info)
            .field("load", &self.load)
            .field("load_xpm_data", &self.load_xpm_data)
            .field("begin_load", &self.begin_load)
            .field("stop_load", &self.stop_load)
            .field("load_increment", &self.load_increment)
            .field("load_animation", &self.load_animation)
            .field("save", &self.save)
            .field("save_to_callback", &self.save_to_callback)
            .field("is_save_option_supported", &self.is_save_option_supported)
            .field("_reserved1", &self._reserved1)
            .field("_reserved2", &self._reserved2)
            .field("_reserved3", &self._reserved3)
            .field("_reserved4", &self._reserved4)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GdkPixbufModulePattern {
    pub prefix: *mut c_char,
    pub mask: *mut c_char,
    pub relevance: c_int,
}

impl ::std::fmt::Debug for GdkPixbufModulePattern {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufModulePattern @ {self:p}"))
            .field("prefix", &self.prefix)
            .field("mask", &self.mask)
            .field("relevance", &self.relevance)
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _GdkPixbufSimpleAnimClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkPixbufSimpleAnimClass = _GdkPixbufSimpleAnimClass;

// Classes
#[repr(C)]
#[allow(dead_code)]
pub struct GdkPixbuf {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkPixbuf {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbuf @ {self:p}")).finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GdkPixbufAnimation {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for GdkPixbufAnimation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufAnimation @ {self:p}"))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GdkPixbufAnimationIter {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for GdkPixbufAnimationIter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufAnimationIter @ {self:p}"))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GdkPixbufLoader {
    pub parent_instance: gobject::GObject,
    pub priv_: gpointer,
}

impl ::std::fmt::Debug for GdkPixbufLoader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufLoader @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GdkPixbufNonAnim {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkPixbufNonAnim {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufNonAnim @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GdkPixbufSimpleAnim {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkPixbufSimpleAnim {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufSimpleAnim @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GdkPixbufSimpleAnimIter {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkPixbufSimpleAnimIter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkPixbufSimpleAnimIter @ {self:p}"))
            .finish()
    }
}

extern "C" {

    //=========================================================================
    // GdkColorspace
    //=========================================================================
    pub fn gdk_colorspace_get_type() -> GType;

    //=========================================================================
    // GdkInterpType
    //=========================================================================
    pub fn gdk_interp_type_get_type() -> GType;

    //=========================================================================
    // GdkPixbufAlphaMode
    //=========================================================================
    pub fn gdk_pixbuf_alpha_mode_get_type() -> GType;

    //=========================================================================
    // GdkPixbufError
    //=========================================================================
    pub fn gdk_pixbuf_error_get_type() -> GType;
    pub fn gdk_pixbuf_error_quark() -> glib::GQuark;

    //=========================================================================
    // GdkPixbufRotation
    //=========================================================================
    pub fn gdk_pixbuf_rotation_get_type() -> GType;

    //=========================================================================
    // GdkPixbufFormat
    //=========================================================================
    pub fn gdk_pixbuf_format_get_type() -> GType;
    pub fn gdk_pixbuf_format_copy(format: *const GdkPixbufFormat) -> *mut GdkPixbufFormat;
    pub fn gdk_pixbuf_format_free(format: *mut GdkPixbufFormat);
    pub fn gdk_pixbuf_format_get_description(format: *mut GdkPixbufFormat) -> *mut c_char;
    pub fn gdk_pixbuf_format_get_extensions(format: *mut GdkPixbufFormat) -> *mut *mut c_char;
    pub fn gdk_pixbuf_format_get_license(format: *mut GdkPixbufFormat) -> *mut c_char;
    pub fn gdk_pixbuf_format_get_mime_types(format: *mut GdkPixbufFormat) -> *mut *mut c_char;
    pub fn gdk_pixbuf_format_get_name(format: *mut GdkPixbufFormat) -> *mut c_char;
    pub fn gdk_pixbuf_format_is_disabled(format: *mut GdkPixbufFormat) -> gboolean;
    pub fn gdk_pixbuf_format_is_save_option_supported(
        format: *mut GdkPixbufFormat,
        option_key: *const c_char,
    ) -> gboolean;
    pub fn gdk_pixbuf_format_is_scalable(format: *mut GdkPixbufFormat) -> gboolean;
    pub fn gdk_pixbuf_format_is_writable(format: *mut GdkPixbufFormat) -> gboolean;
    pub fn gdk_pixbuf_format_set_disabled(format: *mut GdkPixbufFormat, disabled: gboolean);

    //=========================================================================
    // GdkPixbuf
    //=========================================================================
    pub fn gdk_pixbuf_get_type() -> GType;
    pub fn gdk_pixbuf_new(
        colorspace: GdkColorspace,
        has_alpha: gboolean,
        bits_per_sample: c_int,
        width: c_int,
        height: c_int,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_bytes(
        data: *mut glib::GBytes,
        colorspace: GdkColorspace,
        has_alpha: gboolean,
        bits_per_sample: c_int,
        width: c_int,
        height: c_int,
        rowstride: c_int,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_data(
        data: *const u8,
        colorspace: GdkColorspace,
        has_alpha: gboolean,
        bits_per_sample: c_int,
        width: c_int,
        height: c_int,
        rowstride: c_int,
        destroy_fn: GdkPixbufDestroyNotify,
        destroy_fn_data: gpointer,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_file(
        filename: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_file_at_scale(
        filename: *const c_char,
        width: c_int,
        height: c_int,
        preserve_aspect_ratio: gboolean,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_file_at_size(
        filename: *const c_char,
        width: c_int,
        height: c_int,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_inline(
        data_length: c_int,
        data: *const u8,
        copy_pixels: gboolean,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_resource(
        resource_path: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_resource_at_scale(
        resource_path: *const c_char,
        width: c_int,
        height: c_int,
        preserve_aspect_ratio: gboolean,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_stream(
        stream: *mut gio::GInputStream,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_stream_at_scale(
        stream: *mut gio::GInputStream,
        width: c_int,
        height: c_int,
        preserve_aspect_ratio: gboolean,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_stream_finish(
        async_result: *mut gio::GAsyncResult,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_new_from_xpm_data(data: *mut *const c_char) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_calculate_rowstride(
        colorspace: GdkColorspace,
        has_alpha: gboolean,
        bits_per_sample: c_int,
        width: c_int,
        height: c_int,
    ) -> c_int;
    pub fn gdk_pixbuf_get_file_info(
        filename: *const c_char,
        width: *mut c_int,
        height: *mut c_int,
    ) -> *mut GdkPixbufFormat;
    pub fn gdk_pixbuf_get_file_info_async(
        filename: *const c_char,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn gdk_pixbuf_get_file_info_finish(
        async_result: *mut gio::GAsyncResult,
        width: *mut c_int,
        height: *mut c_int,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbufFormat;
    pub fn gdk_pixbuf_get_formats() -> *mut glib::GSList;
    #[cfg(feature = "v2_40")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_40")))]
    pub fn gdk_pixbuf_init_modules(path: *const c_char, error: *mut *mut glib::GError) -> gboolean;
    pub fn gdk_pixbuf_new_from_stream_async(
        stream: *mut gio::GInputStream,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn gdk_pixbuf_new_from_stream_at_scale_async(
        stream: *mut gio::GInputStream,
        width: c_int,
        height: c_int,
        preserve_aspect_ratio: gboolean,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn gdk_pixbuf_save_to_stream_finish(
        async_result: *mut gio::GAsyncResult,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn gdk_pixbuf_add_alpha(
        pixbuf: *const GdkPixbuf,
        substitute_color: gboolean,
        r: c_uchar,
        g: c_uchar,
        b: c_uchar,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_apply_embedded_orientation(src: *mut GdkPixbuf) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_composite(
        src: *const GdkPixbuf,
        dest: *mut GdkPixbuf,
        dest_x: c_int,
        dest_y: c_int,
        dest_width: c_int,
        dest_height: c_int,
        offset_x: c_double,
        offset_y: c_double,
        scale_x: c_double,
        scale_y: c_double,
        interp_type: GdkInterpType,
        overall_alpha: c_int,
    );
    pub fn gdk_pixbuf_composite_color(
        src: *const GdkPixbuf,
        dest: *mut GdkPixbuf,
        dest_x: c_int,
        dest_y: c_int,
        dest_width: c_int,
        dest_height: c_int,
        offset_x: c_double,
        offset_y: c_double,
        scale_x: c_double,
        scale_y: c_double,
        interp_type: GdkInterpType,
        overall_alpha: c_int,
        check_x: c_int,
        check_y: c_int,
        check_size: c_int,
        color1: u32,
        color2: u32,
    );
    pub fn gdk_pixbuf_composite_color_simple(
        src: *const GdkPixbuf,
        dest_width: c_int,
        dest_height: c_int,
        interp_type: GdkInterpType,
        overall_alpha: c_int,
        check_size: c_int,
        color1: u32,
        color2: u32,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_copy(pixbuf: *const GdkPixbuf) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_copy_area(
        src_pixbuf: *const GdkPixbuf,
        src_x: c_int,
        src_y: c_int,
        width: c_int,
        height: c_int,
        dest_pixbuf: *mut GdkPixbuf,
        dest_x: c_int,
        dest_y: c_int,
    );
    pub fn gdk_pixbuf_copy_options(
        src_pixbuf: *mut GdkPixbuf,
        dest_pixbuf: *mut GdkPixbuf,
    ) -> gboolean;
    pub fn gdk_pixbuf_fill(pixbuf: *mut GdkPixbuf, pixel: u32);
    pub fn gdk_pixbuf_flip(src: *const GdkPixbuf, horizontal: gboolean) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_get_bits_per_sample(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_byte_length(pixbuf: *const GdkPixbuf) -> size_t;
    pub fn gdk_pixbuf_get_colorspace(pixbuf: *const GdkPixbuf) -> GdkColorspace;
    pub fn gdk_pixbuf_get_has_alpha(pixbuf: *const GdkPixbuf) -> gboolean;
    pub fn gdk_pixbuf_get_height(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_n_channels(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_option(pixbuf: *mut GdkPixbuf, key: *const c_char) -> *const c_char;
    pub fn gdk_pixbuf_get_options(pixbuf: *mut GdkPixbuf) -> *mut glib::GHashTable;
    pub fn gdk_pixbuf_get_pixels(pixbuf: *const GdkPixbuf) -> *mut u8;
    pub fn gdk_pixbuf_get_pixels_with_length(
        pixbuf: *const GdkPixbuf,
        length: *mut c_uint,
    ) -> *mut u8;
    pub fn gdk_pixbuf_get_rowstride(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_get_width(pixbuf: *const GdkPixbuf) -> c_int;
    pub fn gdk_pixbuf_new_subpixbuf(
        src_pixbuf: *mut GdkPixbuf,
        src_x: c_int,
        src_y: c_int,
        width: c_int,
        height: c_int,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_read_pixel_bytes(pixbuf: *const GdkPixbuf) -> *mut glib::GBytes;
    pub fn gdk_pixbuf_read_pixels(pixbuf: *const GdkPixbuf) -> *const u8;
    pub fn gdk_pixbuf_ref(pixbuf: *mut GdkPixbuf) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_remove_option(pixbuf: *mut GdkPixbuf, key: *const c_char) -> gboolean;
    pub fn gdk_pixbuf_rotate_simple(
        src: *const GdkPixbuf,
        angle: GdkPixbufRotation,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_saturate_and_pixelate(
        src: *const GdkPixbuf,
        dest: *mut GdkPixbuf,
        saturation: c_float,
        pixelate: gboolean,
    );
    pub fn gdk_pixbuf_save(
        pixbuf: *mut GdkPixbuf,
        filename: *const c_char,
        type_: *const c_char,
        error: *mut *mut glib::GError,
        ...
    ) -> gboolean;
    pub fn gdk_pixbuf_save_to_buffer(
        pixbuf: *mut GdkPixbuf,
        buffer: *mut *mut u8,
        buffer_size: *mut size_t,
        type_: *const c_char,
        error: *mut *mut glib::GError,
        ...
    ) -> gboolean;
    pub fn gdk_pixbuf_save_to_bufferv(
        pixbuf: *mut GdkPixbuf,
        buffer: *mut *mut u8,
        buffer_size: *mut size_t,
        type_: *const c_char,
        option_keys: *mut *mut c_char,
        option_values: *mut *mut c_char,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn gdk_pixbuf_save_to_callback(
        pixbuf: *mut GdkPixbuf,
        save_func: GdkPixbufSaveFunc,
        user_data: gpointer,
        type_: *const c_char,
        error: *mut *mut glib::GError,
        ...
    ) -> gboolean;
    pub fn gdk_pixbuf_save_to_callbackv(
        pixbuf: *mut GdkPixbuf,
        save_func: GdkPixbufSaveFunc,
        user_data: gpointer,
        type_: *const c_char,
        option_keys: *mut *mut c_char,
        option_values: *mut *mut c_char,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn gdk_pixbuf_save_to_stream(
        pixbuf: *mut GdkPixbuf,
        stream: *mut gio::GOutputStream,
        type_: *const c_char,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
        ...
    ) -> gboolean;
    pub fn gdk_pixbuf_save_to_stream_async(
        pixbuf: *mut GdkPixbuf,
        stream: *mut gio::GOutputStream,
        type_: *const c_char,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
        ...
    );
    pub fn gdk_pixbuf_save_to_streamv(
        pixbuf: *mut GdkPixbuf,
        stream: *mut gio::GOutputStream,
        type_: *const c_char,
        option_keys: *mut *mut c_char,
        option_values: *mut *mut c_char,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn gdk_pixbuf_save_to_streamv_async(
        pixbuf: *mut GdkPixbuf,
        stream: *mut gio::GOutputStream,
        type_: *const c_char,
        option_keys: *mut *mut c_char,
        option_values: *mut *mut c_char,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn gdk_pixbuf_savev(
        pixbuf: *mut GdkPixbuf,
        filename: *const c_char,
        type_: *const c_char,
        option_keys: *mut *mut c_char,
        option_values: *mut *mut c_char,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn gdk_pixbuf_scale(
        src: *const GdkPixbuf,
        dest: *mut GdkPixbuf,
        dest_x: c_int,
        dest_y: c_int,
        dest_width: c_int,
        dest_height: c_int,
        offset_x: c_double,
        offset_y: c_double,
        scale_x: c_double,
        scale_y: c_double,
        interp_type: GdkInterpType,
    );
    pub fn gdk_pixbuf_scale_simple(
        src: *const GdkPixbuf,
        dest_width: c_int,
        dest_height: c_int,
        interp_type: GdkInterpType,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_set_option(
        pixbuf: *mut GdkPixbuf,
        key: *const c_char,
        value: *const c_char,
    ) -> gboolean;
    pub fn gdk_pixbuf_unref(pixbuf: *mut GdkPixbuf);

    //=========================================================================
    // GdkPixbufAnimation
    //=========================================================================
    pub fn gdk_pixbuf_animation_get_type() -> GType;
    pub fn gdk_pixbuf_animation_new_from_file(
        filename: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_new_from_resource(
        resource_path: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_new_from_stream(
        stream: *mut gio::GInputStream,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_new_from_stream_finish(
        async_result: *mut gio::GAsyncResult,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_new_from_stream_async(
        stream: *mut gio::GInputStream,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn gdk_pixbuf_animation_get_height(animation: *mut GdkPixbufAnimation) -> c_int;
    pub fn gdk_pixbuf_animation_get_iter(
        animation: *mut GdkPixbufAnimation,
        start_time: *const glib::GTimeVal,
    ) -> *mut GdkPixbufAnimationIter;
    pub fn gdk_pixbuf_animation_get_static_image(
        animation: *mut GdkPixbufAnimation,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_animation_get_width(animation: *mut GdkPixbufAnimation) -> c_int;
    pub fn gdk_pixbuf_animation_is_static_image(animation: *mut GdkPixbufAnimation) -> gboolean;
    pub fn gdk_pixbuf_animation_ref(animation: *mut GdkPixbufAnimation) -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_animation_unref(animation: *mut GdkPixbufAnimation);

    //=========================================================================
    // GdkPixbufAnimationIter
    //=========================================================================
    pub fn gdk_pixbuf_animation_iter_get_type() -> GType;
    pub fn gdk_pixbuf_animation_iter_advance(
        iter: *mut GdkPixbufAnimationIter,
        current_time: *const glib::GTimeVal,
    ) -> gboolean;
    pub fn gdk_pixbuf_animation_iter_get_delay_time(iter: *mut GdkPixbufAnimationIter) -> c_int;
    pub fn gdk_pixbuf_animation_iter_get_pixbuf(
        iter: *mut GdkPixbufAnimationIter,
    ) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_animation_iter_on_currently_loading_frame(
        iter: *mut GdkPixbufAnimationIter,
    ) -> gboolean;

    //=========================================================================
    // GdkPixbufLoader
    //=========================================================================
    pub fn gdk_pixbuf_loader_get_type() -> GType;
    pub fn gdk_pixbuf_loader_new() -> *mut GdkPixbufLoader;
    pub fn gdk_pixbuf_loader_new_with_mime_type(
        mime_type: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbufLoader;
    pub fn gdk_pixbuf_loader_new_with_type(
        image_type: *const c_char,
        error: *mut *mut glib::GError,
    ) -> *mut GdkPixbufLoader;
    pub fn gdk_pixbuf_loader_close(
        loader: *mut GdkPixbufLoader,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn gdk_pixbuf_loader_get_animation(loader: *mut GdkPixbufLoader)
        -> *mut GdkPixbufAnimation;
    pub fn gdk_pixbuf_loader_get_format(loader: *mut GdkPixbufLoader) -> *mut GdkPixbufFormat;
    pub fn gdk_pixbuf_loader_get_pixbuf(loader: *mut GdkPixbufLoader) -> *mut GdkPixbuf;
    pub fn gdk_pixbuf_loader_set_size(loader: *mut GdkPixbufLoader, width: c_int, height: c_int);
    pub fn gdk_pixbuf_loader_write(
        loader: *mut GdkPixbufLoader,
        buf: *const u8,
        count: size_t,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn gdk_pixbuf_loader_write_bytes(
        loader: *mut GdkPixbufLoader,
        buffer: *mut glib::GBytes,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // GdkPixbufNonAnim
    //=========================================================================
    pub fn gdk_pixbuf_non_anim_get_type() -> GType;
    pub fn gdk_pixbuf_non_anim_new(pixbuf: *mut GdkPixbuf) -> *mut GdkPixbufAnimation;

    //=========================================================================
    // GdkPixbufSimpleAnim
    //=========================================================================
    pub fn gdk_pixbuf_simple_anim_get_type() -> GType;
    pub fn gdk_pixbuf_simple_anim_new(
        width: c_int,
        height: c_int,
        rate: c_float,
    ) -> *mut GdkPixbufSimpleAnim;
    pub fn gdk_pixbuf_simple_anim_add_frame(
        animation: *mut GdkPixbufSimpleAnim,
        pixbuf: *mut GdkPixbuf,
    );
    pub fn gdk_pixbuf_simple_anim_get_loop(animation: *mut GdkPixbufSimpleAnim) -> gboolean;
    pub fn gdk_pixbuf_simple_anim_set_loop(animation: *mut GdkPixbufSimpleAnim, loop_: gboolean);

    //=========================================================================
    // GdkPixbufSimpleAnimIter
    //=========================================================================
    pub fn gdk_pixbuf_simple_anim_iter_get_type() -> GType;

}
