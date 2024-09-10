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

use cairo_sys as cairo;
use glib_sys as glib;
use pango_sys as pango;

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

// Callbacks
pub type PangoCairoShapeRendererFunc = Option<
    unsafe extern "C" fn(*mut cairo::cairo_t, *mut pango::PangoAttrShape, gboolean, gpointer),
>;

// Interfaces
#[repr(C)]
#[allow(dead_code)]
pub struct PangoCairoFont {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PangoCairoFont {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "PangoCairoFont @ {self:p}")
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct PangoCairoFontMap {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PangoCairoFontMap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "PangoCairoFontMap @ {self:p}")
    }
}

extern "C" {

    //=========================================================================
    // PangoCairoFont
    //=========================================================================
    pub fn pango_cairo_font_get_type() -> GType;
    pub fn pango_cairo_font_get_scaled_font(
        font: *mut PangoCairoFont,
    ) -> *mut cairo::cairo_scaled_font_t;

    //=========================================================================
    // PangoCairoFontMap
    //=========================================================================
    pub fn pango_cairo_font_map_get_type() -> GType;
    pub fn pango_cairo_font_map_get_default() -> *mut pango::PangoFontMap;
    pub fn pango_cairo_font_map_new() -> *mut pango::PangoFontMap;
    pub fn pango_cairo_font_map_new_for_font_type(
        fonttype: cairo::cairo_font_type_t,
    ) -> *mut pango::PangoFontMap;
    pub fn pango_cairo_font_map_create_context(
        fontmap: *mut PangoCairoFontMap,
    ) -> *mut pango::PangoContext;
    pub fn pango_cairo_font_map_get_font_type(
        fontmap: *mut PangoCairoFontMap,
    ) -> cairo::cairo_font_type_t;
    pub fn pango_cairo_font_map_get_resolution(fontmap: *mut PangoCairoFontMap) -> c_double;
    pub fn pango_cairo_font_map_set_default(fontmap: *mut PangoCairoFontMap);
    pub fn pango_cairo_font_map_set_resolution(fontmap: *mut PangoCairoFontMap, dpi: c_double);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn pango_cairo_context_get_font_options(
        context: *mut pango::PangoContext,
    ) -> *const cairo::cairo_font_options_t;
    pub fn pango_cairo_context_get_resolution(context: *mut pango::PangoContext) -> c_double;
    pub fn pango_cairo_context_get_shape_renderer(
        context: *mut pango::PangoContext,
        data: *mut gpointer,
    ) -> PangoCairoShapeRendererFunc;
    pub fn pango_cairo_context_set_font_options(
        context: *mut pango::PangoContext,
        options: *const cairo::cairo_font_options_t,
    );
    pub fn pango_cairo_context_set_resolution(context: *mut pango::PangoContext, dpi: c_double);
    pub fn pango_cairo_context_set_shape_renderer(
        context: *mut pango::PangoContext,
        func: PangoCairoShapeRendererFunc,
        data: gpointer,
        dnotify: glib::GDestroyNotify,
    );
    pub fn pango_cairo_create_context(cr: *mut cairo::cairo_t) -> *mut pango::PangoContext;
    pub fn pango_cairo_create_layout(cr: *mut cairo::cairo_t) -> *mut pango::PangoLayout;
    pub fn pango_cairo_error_underline_path(
        cr: *mut cairo::cairo_t,
        x: c_double,
        y: c_double,
        width: c_double,
        height: c_double,
    );
    pub fn pango_cairo_glyph_string_path(
        cr: *mut cairo::cairo_t,
        font: *mut pango::PangoFont,
        glyphs: *mut pango::PangoGlyphString,
    );
    pub fn pango_cairo_layout_line_path(cr: *mut cairo::cairo_t, line: *mut pango::PangoLayoutLine);
    pub fn pango_cairo_layout_path(cr: *mut cairo::cairo_t, layout: *mut pango::PangoLayout);
    pub fn pango_cairo_show_error_underline(
        cr: *mut cairo::cairo_t,
        x: c_double,
        y: c_double,
        width: c_double,
        height: c_double,
    );
    pub fn pango_cairo_show_glyph_item(
        cr: *mut cairo::cairo_t,
        text: *const c_char,
        glyph_item: *mut pango::PangoGlyphItem,
    );
    pub fn pango_cairo_show_glyph_string(
        cr: *mut cairo::cairo_t,
        font: *mut pango::PangoFont,
        glyphs: *mut pango::PangoGlyphString,
    );
    pub fn pango_cairo_show_layout(cr: *mut cairo::cairo_t, layout: *mut pango::PangoLayout);
    pub fn pango_cairo_show_layout_line(cr: *mut cairo::cairo_t, line: *mut pango::PangoLayoutLine);
    pub fn pango_cairo_update_context(cr: *mut cairo::cairo_t, context: *mut pango::PangoContext);
    pub fn pango_cairo_update_layout(cr: *mut cairo::cairo_t, layout: *mut pango::PangoLayout);

}
