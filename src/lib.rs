#![allow(non_camel_case_types, non_snake_case)]
use std::ffi::{CString, CStr};
use std::path::Path;
mod ffi;
mod urls;

pub use urls::*;

pub struct TaobaoClient {
    ptr: ffi::pTaobaoClient,
}

impl TaobaoClient {
    pub fn new<S: AsRef<str>>(url: S, appkey: S, secret: S) -> TaobaoClient {
        let url = CString::new(url.as_ref()).unwrap();
        let appkey = CString::new(appkey.as_ref()).unwrap();
        let secret = CString::new(secret.as_ref()).unwrap();
        TaobaoClient {
            ptr: unsafe {
                ffi::alloc_taobao_client(url.as_ptr(), appkey.as_ptr(), secret.as_ptr())
            },
        }
    }

    pub fn execute<S: AsRef<str>>(&mut self,
                                  request: &mut TopRequest,
                                  session: S)
                                  -> Result<TopResponseIterator, ()> {
        let session = CString::new(session.as_ref()).unwrap();
        let session_ptr = session.into_raw();
        unsafe {
            let mut response =
                TopResponse::from_raw(ffi::top_execute(self.ptr, request.ptr(), session_ptr));
            CString::from_raw(session_ptr);

            if response.code() == 0 {
                Ok(TopResponseIterator::from_response(&mut response))
            } else {
                Err(())
            }
        }
    }
}

impl Drop for TaobaoClient {
    fn drop(&mut self) {
        unsafe {
            ffi::destroy_taobao_client(self.ptr);
        }
    }
}

pub struct TopRequest {
    ptr: ffi::pTopRequest,
}

macro_rules! param_fn {
    ($name:ident) => {
        pub fn $name<S: AsRef<str>>(&mut self, key: S, value: S) -> Result<(), ()> {
            let key = CString::new(key.as_ref()).unwrap();
            let value = CString::new(value.as_ref()).unwrap();
            unsafe {
                match ffi::$name(self.ptr, key.as_ptr(), value.as_ptr()) {
                    0 => Ok(()),
                    _ => Err(()),
                }
            }
        }
    }
}

impl TopRequest {
    pub fn new() -> TopRequest {
        TopRequest { ptr: unsafe { ffi::alloc_top_request() } }
    }

    pub fn set_api_name<S: AsRef<str>>(&mut self, name: S) -> Result<(), ()> {
        let name = CString::new(name.as_ref()).unwrap();
        unsafe {
            match ffi::set_api_name(self.ptr, name.as_ptr()) {
                0 => Ok(()),
                _ => Err(()),
            }
        }
    }

    param_fn!(add_httpheader_add_param);
    param_fn!(add_param);
    param_fn!(add_file_param);

    fn ptr(&mut self) -> ffi::pTopRequest {
        self.ptr
    }
}

impl Drop for TopRequest {
    fn drop(&mut self) {
        unsafe {
            ffi::destroy_top_request(self.ptr);
        }
    }
}

pub struct TopResponse {
    ptr: ffi::pTopResponse,
}

impl TopResponse {
    pub fn new() -> TopResponse {
        TopResponse { ptr: unsafe { ffi::alloc_top_response() } }
    }

    fn from_raw(raw: ffi::pTopResponse) -> TopResponse {
        TopResponse { ptr: raw }
    }

    fn code(&self) -> ::std::os::raw::c_int {
        unsafe { (*self.ptr).code }
    }

    fn ptr(&mut self) -> ffi::pTopResponse {
        self.ptr
    }
}

impl Drop for TopResponse {
    fn drop(&mut self) {
        unsafe {
            ffi::destroy_top_response(self.ptr);
        }
    }
}

pub struct TopResponseIterator<'a> {
    ptr: ffi::pTopResponseIterator,
    marker: ::std::marker::PhantomData<&'a str>,
}

impl<'a> TopResponseIterator<'a> {
    fn from_response(response: &mut TopResponse) -> TopResponseIterator<'a> {
        TopResponseIterator {
            ptr: unsafe { ffi::init_response_iterator(response.ptr()) },
            marker: ::std::marker::PhantomData,
        }
    }
}

impl<'a> Drop for TopResponseIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::destroy_response_iterator(self.ptr);
        }
    }
}

impl<'a> Iterator for TopResponseIterator<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<(&'a str, &'a str)> {
        unsafe {
            let resultitem = ffi::alloc_result_item();
            if ffi::parseNext(self.ptr, resultitem) == 0 {
                let resultitem = ResultItem::from_raw(resultitem);
                Some((resultitem.key, resultitem.value))
            } else {
                None
            }
        }
    }
}

struct ResultItem<'a> {
    pub key: &'a str,
    pub value: &'a str,
    ptr: ffi::pResultItem,
}

impl<'a> ResultItem<'a> {
    fn from_raw(raw: ffi::pResultItem) -> ResultItem<'a> {
        unsafe {
            ResultItem {
                key: CStr::from_ptr((*raw).key).to_str().unwrap(),
                value: CStr::from_ptr((*raw).value).to_str().unwrap(),
                ptr: raw,
            }
        }
    }
}

impl<'a> Drop for ResultItem<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::destroy_result_item(self.ptr);
        }
    }
}

pub fn set_capath<P: AsRef<Path>>(path: P) {
    let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
    let path_ptr = path.into_raw();
    unsafe {
        ffi::set_capath(path_ptr);
        CString::from_raw(path_ptr);
    }
}

macro_rules! int_cast_fn {
    ($name:ident, $param:ident) => {
        pub fn $name($param:u32) {
            unsafe {
                ffi::$name($param as ::std::os::raw::c_int);
            }
        }
    }
}

int_cast_fn!(set_retry_times, retry);
int_cast_fn!(set_retry_sleep_times, sleep_time);
int_cast_fn!(set_http_time_out, timeout);