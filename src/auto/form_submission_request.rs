// This file was generated by gir (e95e5d8) from gir-files (857b8f5)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FormSubmissionRequest(Object<ffi::WebKitFormSubmissionRequest>);

    match fn {
        get_type => || ffi::webkit_form_submission_request_get_type(),
    }
}

pub trait FormSubmissionRequestExt {
    //fn get_text_fields(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 };

    fn submit(&self);
}

impl<O: IsA<FormSubmissionRequest>> FormSubmissionRequestExt for O {
    //fn get_text_fields(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 } {
    //    unsafe { TODO: call ffi::webkit_form_submission_request_get_text_fields() }
    //}

    fn submit(&self) {
        unsafe {
            ffi::webkit_form_submission_request_submit(self.to_glib_none().0);
        }
    }
}
