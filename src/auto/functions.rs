// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Atom;
use Cursor;
use Display;
use Event;
use EventMask;
use EventType;
use GrabStatus;
use ModifierType;
use Screen;
use Visual;
use Window;
use WindowState;
use cairo;
use ffi;
use gdk_pixbuf;
use glib::translate::*;
use libc;
use pango;
use std::mem;
use std::ptr;


//#[cfg_attr(feature = "v3_16", deprecated)]
//pub fn add_option_entries_libgtk_only(group: /*Ignored*/&glib::OptionGroup) {
//    unsafe { TODO: call ffi::gdk_add_option_entries_libgtk_only() }
//}

pub fn beep() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_beep();
    }
}

pub fn error_trap_pop() -> i32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_error_trap_pop()
    }
}

pub fn error_trap_pop_ignored() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_error_trap_pop_ignored();
    }
}

pub fn error_trap_push() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_error_trap_push();
    }
}

pub fn events_get_angle(event1: &mut Event, event2: &mut Event) -> Option<f64> {
    assert_initialized_main_thread!();
    unsafe {
        let mut angle = mem::uninitialized();
        let ret = from_glib(ffi::gdk_events_get_angle(event1.to_glib_none_mut().0, event2.to_glib_none_mut().0, &mut angle));
        if ret { Some(angle) } else { None }
    }
}

pub fn events_get_center(event1: &mut Event, event2: &mut Event) -> Option<(f64, f64)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut x = mem::uninitialized();
        let mut y = mem::uninitialized();
        let ret = from_glib(ffi::gdk_events_get_center(event1.to_glib_none_mut().0, event2.to_glib_none_mut().0, &mut x, &mut y));
        if ret { Some((x, y)) } else { None }
    }
}

pub fn events_get_distance(event1: &mut Event, event2: &mut Event) -> Option<f64> {
    assert_initialized_main_thread!();
    unsafe {
        let mut distance = mem::uninitialized();
        let ret = from_glib(ffi::gdk_events_get_distance(event1.to_glib_none_mut().0, event2.to_glib_none_mut().0, &mut distance));
        if ret { Some(distance) } else { None }
    }
}

pub fn events_pending() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gdk_events_pending())
    }
}

pub fn flush() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_flush();
    }
}

#[cfg_attr(feature = "v3_8", deprecated)]
pub fn get_display() -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gdk_get_display())
    }
}

pub fn get_display_arg_name() -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gdk_get_display_arg_name())
    }
}

pub fn get_program_class() -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gdk_get_program_class())
    }
}

pub fn get_show_events() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gdk_get_show_events())
    }
}

//pub fn init_check(argv: /*Unimplemented*/Vec<String>) -> bool {
//    unsafe { TODO: call ffi::gdk_init_check() }
//}

#[deprecated]
pub fn keyboard_grab(window: &Window, owner_events: bool, time_: u32) -> GrabStatus {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gdk_keyboard_grab(window.to_glib_none().0, owner_events.to_glib(), time_))
    }
}

#[deprecated]
pub fn keyboard_ungrab(time_: u32) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_keyboard_ungrab(time_);
    }
}

pub fn keyval_convert_case(symbol: u32) -> (u32, u32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut lower = mem::uninitialized();
        let mut upper = mem::uninitialized();
        ffi::gdk_keyval_convert_case(symbol, &mut lower, &mut upper);
        (lower, upper)
    }
}

pub fn keyval_from_name(keyval_name: &str) -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_keyval_from_name(keyval_name.to_glib_none().0)
    }
}

pub fn keyval_is_lower(keyval: u32) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gdk_keyval_is_lower(keyval))
    }
}

pub fn keyval_is_upper(keyval: u32) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gdk_keyval_is_upper(keyval))
    }
}

pub fn keyval_to_lower(keyval: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_keyval_to_lower(keyval)
    }
}

pub fn keyval_to_upper(keyval: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_keyval_to_upper(keyval)
    }
}

#[cfg_attr(feature = "v3_22", deprecated)]
pub fn list_visuals() -> Vec<Visual> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_container(ffi::gdk_list_visuals())
    }
}

pub fn notify_startup_complete() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_notify_startup_complete();
    }
}

pub fn notify_startup_complete_with_id(startup_id: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_notify_startup_complete_with_id(startup_id.to_glib_none().0);
    }
}

pub fn pango_context_get() -> Option<pango::Context> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gdk_pango_context_get())
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
pub fn pango_context_get_for_display(display: &Display) -> Option<pango::Context> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gdk_pango_context_get_for_display(display.to_glib_none().0))
    }
}

pub fn pango_context_get_for_screen(screen: &Screen) -> Option<pango::Context> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gdk_pango_context_get_for_screen(screen.to_glib_none().0))
    }
}

//pub fn pango_layout_get_clip_region(layout: /*Ignored*/&pango::Layout, x_origin: i32, y_origin: i32, index_ranges: i32, n_ranges: i32) -> Option<cairo::Region> {
//    unsafe { TODO: call ffi::gdk_pango_layout_get_clip_region() }
//}

//pub fn pango_layout_line_get_clip_region(line: /*Ignored*/&pango::LayoutLine, x_origin: i32, y_origin: i32, index_ranges: &[i32], n_ranges: i32) -> Option<cairo::Region> {
//    unsafe { TODO: call ffi::gdk_pango_layout_line_get_clip_region() }
//}

//pub fn parse_args(argv: /*Unimplemented*/Vec<String>) {
//    unsafe { TODO: call ffi::gdk_parse_args() }
//}

pub fn pixbuf_get_from_surface(surface: &cairo::Surface, src_x: i32, src_y: i32, width: i32, height: i32) -> Option<gdk_pixbuf::Pixbuf> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gdk_pixbuf_get_from_surface(mut_override(surface.to_glib_none().0), src_x, src_y, width, height))
    }
}

#[deprecated]
pub fn pointer_grab<'a, 'b, P: Into<Option<&'a Window>>, Q: Into<Option<&'b Cursor>>>(window: &Window, owner_events: bool, event_mask: EventMask, confine_to: P, cursor: Q, time_: u32) -> GrabStatus {
    skip_assert_initialized!();
    let confine_to = confine_to.into();
    let confine_to = confine_to.to_glib_none();
    let cursor = cursor.into();
    let cursor = cursor.to_glib_none();
    unsafe {
        from_glib(ffi::gdk_pointer_grab(window.to_glib_none().0, owner_events.to_glib(), event_mask.to_glib(), confine_to.0, cursor.0, time_))
    }
}

#[deprecated]
pub fn pointer_is_grabbed() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gdk_pointer_is_grabbed())
    }
}

#[deprecated]
pub fn pointer_ungrab(time_: u32) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_pointer_ungrab(time_);
    }
}

#[cfg_attr(feature = "v3_16", deprecated)]
pub fn pre_parse_libgtk_only() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_pre_parse_libgtk_only();
    }
}

//pub fn property_change(window: &Window, property: &Atom, type_: &Atom, format: i32, mode: /*Ignored*/PropMode, data: u8, nelements: i32) {
//    unsafe { TODO: call ffi::gdk_property_change() }
//}

pub fn property_delete(window: &Window, property: &Atom) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_property_delete(window.to_glib_none().0, property.to_glib_none().0);
    }
}

pub fn property_get(window: &Window, property: &Atom, type_: &Atom, offset: libc::c_ulong, length: libc::c_ulong, pdelete: i32) -> Option<(Atom, i32, Vec<u8>)> {
    skip_assert_initialized!();
    unsafe {
        let mut actual_property_type = Atom::uninitialized();
        let mut actual_format = mem::uninitialized();
        let mut actual_length = mem::uninitialized();
        let mut data = ptr::null_mut();
        let ret = from_glib(ffi::gdk_property_get(window.to_glib_none().0, property.to_glib_none().0, type_.to_glib_none().0, offset, length, pdelete, actual_property_type.to_glib_none_mut().0, &mut actual_format, &mut actual_length, &mut data));
        if ret { Some((actual_property_type, actual_format, FromGlibContainer::from_glib_full_num(data, actual_length as usize))) } else { None }
    }
}

#[cfg_attr(feature = "v3_22", deprecated)]
pub fn query_depths() -> Vec<i32> {
    assert_initialized_main_thread!();
    unsafe {
        let mut depths = ptr::null_mut();
        let mut count = mem::uninitialized();
        ffi::gdk_query_depths(&mut depths, &mut count);
        FromGlibContainer::from_glib_none_num(depths, count as usize)
    }
}

//#[cfg_attr(feature = "v3_22", deprecated)]
//pub fn query_visual_types(visual_types: /*Unimplemented*/CArray TypeId { ns_id: 1, id: 99 }) -> i32 {
//    unsafe { TODO: call ffi::gdk_query_visual_types() }
//}

pub fn selection_convert(requestor: &Window, selection: &Atom, target: &Atom, time_: u32) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_selection_convert(requestor.to_glib_none().0, selection.to_glib_none().0, target.to_glib_none().0, time_);
    }
}

pub fn selection_owner_get(selection: &Atom) -> Option<Window> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gdk_selection_owner_get(selection.to_glib_none().0))
    }
}

pub fn selection_owner_get_for_display(display: &Display, selection: &Atom) -> Option<Window> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gdk_selection_owner_get_for_display(display.to_glib_none().0, selection.to_glib_none().0))
    }
}

pub fn selection_owner_set<'a, P: Into<Option<&'a Window>>>(owner: P, selection: &Atom, time_: u32, send_event: bool) -> bool {
    assert_initialized_main_thread!();
    let owner = owner.into();
    let owner = owner.to_glib_none();
    unsafe {
        from_glib(ffi::gdk_selection_owner_set(owner.0, selection.to_glib_none().0, time_, send_event.to_glib()))
    }
}

pub fn selection_owner_set_for_display<'a, P: Into<Option<&'a Window>>>(display: &Display, owner: P, selection: &Atom, time_: u32, send_event: bool) -> bool {
    skip_assert_initialized!();
    let owner = owner.into();
    let owner = owner.to_glib_none();
    unsafe {
        from_glib(ffi::gdk_selection_owner_set_for_display(display.to_glib_none().0, owner.0, selection.to_glib_none().0, time_, send_event.to_glib()))
    }
}

pub fn selection_send_notify(requestor: &Window, selection: &Atom, target: &Atom, property: &Atom, time_: u32) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_selection_send_notify(requestor.to_glib_none().0, selection.to_glib_none().0, target.to_glib_none().0, property.to_glib_none().0, time_);
    }
}

pub fn selection_send_notify_for_display(display: &Display, requestor: &Window, selection: &Atom, target: &Atom, property: &Atom, time_: u32) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_selection_send_notify_for_display(display.to_glib_none().0, requestor.to_glib_none().0, selection.to_glib_none().0, target.to_glib_none().0, property.to_glib_none().0, time_);
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
pub fn set_allowed_backends(backends: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_set_allowed_backends(backends.to_glib_none().0);
    }
}

pub fn set_double_click_time(msec: u32) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_set_double_click_time(msec);
    }
}

pub fn set_program_class(program_class: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_set_program_class(program_class.to_glib_none().0);
    }
}

pub fn set_show_events(show_events: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_set_show_events(show_events.to_glib());
    }
}

//pub fn setting_get(name: &str, value: /*Ignored*/&mut glib::Value) -> bool {
//    unsafe { TODO: call ffi::gdk_setting_get() }
//}

pub fn synthesize_window_state(window: &Window, unset_flags: WindowState, set_flags: WindowState) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_synthesize_window_state(window.to_glib_none().0, unset_flags.to_glib(), set_flags.to_glib());
    }
}

pub fn test_render_sync(window: &Window) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_test_render_sync(window.to_glib_none().0);
    }
}

pub fn test_simulate_button(window: &Window, x: i32, y: i32, button: u32, modifiers: ModifierType, button_pressrelease: EventType) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gdk_test_simulate_button(window.to_glib_none().0, x, y, button, modifiers.to_glib(), button_pressrelease.to_glib()))
    }
}

pub fn test_simulate_key(window: &Window, x: i32, y: i32, keyval: u32, modifiers: ModifierType, key_pressrelease: EventType) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gdk_test_simulate_key(window.to_glib_none().0, x, y, keyval, modifiers.to_glib(), key_pressrelease.to_glib()))
    }
}

pub fn text_property_to_utf8_list_for_display(display: &Display, encoding: &Atom, format: i32, text: &[u8]) -> (i32, Vec<String>) {
    skip_assert_initialized!();
    let length = text.len() as i32;
    unsafe {
        let mut list = ptr::null_mut();
        let ret = ffi::gdk_text_property_to_utf8_list_for_display(display.to_glib_none().0, encoding.to_glib_none().0, format, text.to_glib_none().0, length, &mut list);
        (ret, FromGlibPtrContainer::from_glib_full(list))
    }
}

//pub fn threads_add_idle<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(function: /*Unknown conversion*//*Unimplemented*/SourceFunc, data: P) -> u32 {
//    unsafe { TODO: call ffi::gdk_threads_add_idle() }
//}

//pub fn threads_add_idle_full<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(priority: i32, function: /*Unknown conversion*//*Unimplemented*/SourceFunc, data: P, notify: Q) -> u32 {
//    unsafe { TODO: call ffi::gdk_threads_add_idle_full() }
//}

//pub fn threads_add_timeout<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(interval: u32, function: /*Unknown conversion*//*Unimplemented*/SourceFunc, data: P) -> u32 {
//    unsafe { TODO: call ffi::gdk_threads_add_timeout() }
//}

//pub fn threads_add_timeout_full<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(priority: i32, interval: u32, function: /*Unknown conversion*//*Unimplemented*/SourceFunc, data: P, notify: Q) -> u32 {
//    unsafe { TODO: call ffi::gdk_threads_add_timeout_full() }
//}

//pub fn threads_add_timeout_seconds<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(interval: u32, function: /*Unknown conversion*//*Unimplemented*/SourceFunc, data: P) -> u32 {
//    unsafe { TODO: call ffi::gdk_threads_add_timeout_seconds() }
//}

//pub fn threads_add_timeout_seconds_full<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(priority: i32, interval: u32, function: /*Unknown conversion*//*Unimplemented*/SourceFunc, data: P, notify: Q) -> u32 {
//    unsafe { TODO: call ffi::gdk_threads_add_timeout_seconds_full() }
//}

#[cfg_attr(feature = "v3_6", deprecated)]
pub fn threads_enter() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_threads_enter();
    }
}

#[cfg_attr(feature = "v3_6", deprecated)]
pub fn threads_init() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_threads_init();
    }
}

#[cfg_attr(feature = "v3_6", deprecated)]
pub fn threads_leave() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_threads_leave();
    }
}

//#[cfg_attr(feature = "v3_6", deprecated)]
//pub fn threads_set_lock_functions(enter_fn: /*Unknown conversion*//*Unimplemented*/Callback, leave_fn: /*Unknown conversion*//*Unimplemented*/Callback) {
//    unsafe { TODO: call ffi::gdk_threads_set_lock_functions() }
//}

pub fn unicode_to_keyval(wc: u32) -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_unicode_to_keyval(wc)
    }
}

pub fn utf8_to_string_target(str: &str) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gdk_utf8_to_string_target(str.to_glib_none().0))
    }
}
