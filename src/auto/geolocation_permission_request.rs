// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use PermissionRequest;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct GeolocationPermissionRequest(Object<ffi::WebKitGeolocationPermissionRequest>): PermissionRequest;

    match fn {
        get_type => || ffi::webkit_geolocation_permission_request_get_type(),
    }
}

impl GeolocationPermissionRequest {}
