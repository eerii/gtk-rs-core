// This file was generated by gir (c9185c9) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use Container;
use PackType;
use PositionType;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Notebook(Object<ffi::GtkNotebook>): Widget, Container, Buildable;

    match fn {
        get_type => || ffi::gtk_notebook_get_type(),
    }
}

impl Notebook {
    pub fn new() -> Notebook {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_notebook_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_16)]
    pub fn detach_tab<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_notebook_detach_tab(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    pub fn get_action_widget(&self, pack_type: PackType) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_action_widget(self.to_glib_none().0, pack_type))
        }
    }

    pub fn get_group_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_group_name(self.to_glib_none().0))
        }
    }

    pub fn get_menu_label<T: IsA<Widget>>(&self, child: &T) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_menu_label_text<T: IsA<Widget>>(&self, child: &T) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label_text(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_scrollable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_scrollable(self.to_glib_none().0))
        }
    }

    pub fn get_show_border(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_show_border(self.to_glib_none().0))
        }
    }

    pub fn get_show_tabs(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_show_tabs(self.to_glib_none().0))
        }
    }

    pub fn get_tab_detachable<T: IsA<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_detachable(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_tab_hborder(&self) -> u16 {
        unsafe {
            ffi::gtk_notebook_get_tab_hborder(self.to_glib_none().0)
        }
    }

    pub fn get_tab_label<T: IsA<Widget>>(&self, child: &T) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_tab_label_text<T: IsA<Widget>>(&self, child: &T) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label_text(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_tab_pos(&self) -> PositionType {
        unsafe {
            ffi::gtk_notebook_get_tab_pos(self.to_glib_none().0)
        }
    }

    pub fn get_tab_reorderable<T: IsA<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_reorderable(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    pub fn get_tab_vborder(&self) -> u16 {
        unsafe {
            ffi::gtk_notebook_get_tab_vborder(self.to_glib_none().0)
        }
    }

    pub fn next_page(&self) {
        unsafe {
            ffi::gtk_notebook_next_page(self.to_glib_none().0);
        }
    }

    pub fn popup_disable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_disable(self.to_glib_none().0);
        }
    }

    pub fn popup_enable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_enable(self.to_glib_none().0);
        }
    }

    pub fn prev_page(&self) {
        unsafe {
            ffi::gtk_notebook_prev_page(self.to_glib_none().0);
        }
    }

    pub fn set_action_widget<T: IsA<Widget>>(&self, widget: &T, pack_type: PackType) {
        unsafe {
            ffi::gtk_notebook_set_action_widget(self.to_glib_none().0, widget.to_glib_none().0, pack_type);
        }
    }

    pub fn set_group_name(&self, group_name: Option<&str>) {
        unsafe {
            ffi::gtk_notebook_set_group_name(self.to_glib_none().0, group_name.to_glib_none().0);
        }
    }

    pub fn set_menu_label<T: IsA<Widget>, U: IsA<Widget>>(&self, child: &T, menu_label: Option<&U>) {
        unsafe {
            ffi::gtk_notebook_set_menu_label(self.to_glib_none().0, child.to_glib_none().0, menu_label.to_glib_none().0);
        }
    }

    pub fn set_menu_label_text<T: IsA<Widget>>(&self, child: &T, menu_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_menu_label_text(self.to_glib_none().0, child.to_glib_none().0, menu_text.to_glib_none().0);
        }
    }

    pub fn set_scrollable(&self, scrollable: bool) {
        unsafe {
            ffi::gtk_notebook_set_scrollable(self.to_glib_none().0, scrollable.to_glib());
        }
    }

    pub fn set_show_border(&self, show_border: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_border(self.to_glib_none().0, show_border.to_glib());
        }
    }

    pub fn set_show_tabs(&self, show_tabs: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_tabs(self.to_glib_none().0, show_tabs.to_glib());
        }
    }

    pub fn set_tab_detachable<T: IsA<Widget>>(&self, child: &T, detachable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_detachable(self.to_glib_none().0, child.to_glib_none().0, detachable.to_glib());
        }
    }

    pub fn set_tab_label<T: IsA<Widget>, U: IsA<Widget>>(&self, child: &T, tab_label: Option<&U>) {
        unsafe {
            ffi::gtk_notebook_set_tab_label(self.to_glib_none().0, child.to_glib_none().0, tab_label.to_glib_none().0);
        }
    }

    pub fn set_tab_label_text<T: IsA<Widget>>(&self, child: &T, tab_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_tab_label_text(self.to_glib_none().0, child.to_glib_none().0, tab_text.to_glib_none().0);
        }
    }

    pub fn set_tab_pos(&self, pos: PositionType) {
        unsafe {
            ffi::gtk_notebook_set_tab_pos(self.to_glib_none().0, pos);
        }
    }

    pub fn set_tab_reorderable<T: IsA<Widget>>(&self, child: &T, reorderable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_reorderable(self.to_glib_none().0, child.to_glib_none().0, reorderable.to_glib());
        }
    }
}
