// This file was generated by gir (e95e5d8) from gir-files (857b8f5)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use java_script_core;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct JavascriptResult(Shared<ffi::WebKitJavascriptResult>);

    match fn {
        ref => |ptr| ffi::webkit_javascript_result_ref(ptr),
        unref => |ptr| ffi::webkit_javascript_result_unref(ptr),
        get_type => || ffi::webkit_javascript_result_get_type(),
    }
}

impl JavascriptResult {
    pub fn get_global_context(&self) -> Option<java_script_core::GlobalContext> {
        unsafe {
            from_glib_full(ffi::webkit_javascript_result_get_global_context(self.to_glib_none().0))
        }
    }

    pub fn get_value(&self) -> Option<java_script_core::Value> {
        unsafe {
            from_glib_full(ffi::webkit_javascript_result_get_value(self.to_glib_none().0))
        }
    }
}
