// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    pub struct Matrix(BoxedInline<ffi::PangoMatrix>);

    match fn {
        copy => |ptr| ffi::pango_matrix_copy(ptr),
        free => |ptr| ffi::pango_matrix_free(ptr),
        type_ => || ffi::pango_matrix_get_type(),
    }
}

impl Matrix {
    #[doc(alias = "pango_matrix_concat")]
    pub fn concat(&mut self, new_matrix: &Matrix) {
        unsafe {
            ffi::pango_matrix_concat(self.to_glib_none_mut().0, new_matrix.to_glib_none().0);
        }
    }

    #[doc(alias = "pango_matrix_get_font_scale_factor")]
    #[doc(alias = "get_font_scale_factor")]
    pub fn font_scale_factor(&self) -> f64 {
        unsafe { ffi::pango_matrix_get_font_scale_factor(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_matrix_get_font_scale_factors")]
    #[doc(alias = "get_font_scale_factors")]
    pub fn font_scale_factors(&self) -> (f64, f64) {
        unsafe {
            let mut xscale = std::mem::MaybeUninit::uninit();
            let mut yscale = std::mem::MaybeUninit::uninit();
            ffi::pango_matrix_get_font_scale_factors(
                self.to_glib_none().0,
                xscale.as_mut_ptr(),
                yscale.as_mut_ptr(),
            );
            (xscale.assume_init(), yscale.assume_init())
        }
    }

    #[cfg(feature = "v1_50")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_matrix_get_slant_ratio")]
    #[doc(alias = "get_slant_ratio")]
    pub fn slant_ratio(&self) -> f64 {
        unsafe { ffi::pango_matrix_get_slant_ratio(self.to_glib_none().0) }
    }

    #[doc(alias = "pango_matrix_rotate")]
    pub fn rotate(&mut self, degrees: f64) {
        unsafe {
            ffi::pango_matrix_rotate(self.to_glib_none_mut().0, degrees);
        }
    }

    #[doc(alias = "pango_matrix_scale")]
    pub fn scale(&mut self, scale_x: f64, scale_y: f64) {
        unsafe {
            ffi::pango_matrix_scale(self.to_glib_none_mut().0, scale_x, scale_y);
        }
    }

    #[doc(alias = "pango_matrix_transform_distance")]
    pub fn transform_distance(&self, dx: &mut f64, dy: &mut f64) {
        unsafe {
            ffi::pango_matrix_transform_distance(self.to_glib_none().0, dx, dy);
        }
    }

    #[doc(alias = "pango_matrix_transform_point")]
    pub fn transform_point(&self, x: &mut f64, y: &mut f64) {
        unsafe {
            ffi::pango_matrix_transform_point(self.to_glib_none().0, x, y);
        }
    }

    #[doc(alias = "pango_matrix_translate")]
    pub fn translate(&mut self, tx: f64, ty: f64) {
        unsafe {
            ffi::pango_matrix_translate(self.to_glib_none_mut().0, tx, ty);
        }
    }
}
