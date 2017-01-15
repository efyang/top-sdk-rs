#![allow(non_camel_case_types, non_snake_case)]
#[macro_use]
extern crate quick_error;
extern crate curl_sys;
use std::ffi::{CString, CStr};
use std::path::Path;

mod error;
mod ffi;
mod urls;

pub use urls::*;
pub use error::*;

pub type Session = String;

pub struct TaobaoClient {
    ptr: ffi::pTaobaoClient,
}

impl TaobaoClient {
    pub fn new<S: AsRef<str>>(url: S, appkey: S, secret: S) -> Result<TaobaoClient, TopError> {
        let url = CString::new(url.as_ref())?;
        let appkey = CString::new(appkey.as_ref())?;
        let secret = CString::new(secret.as_ref())?;
        Ok(TaobaoClient {
            ptr: unsafe {
                ffi::alloc_taobao_client(url.as_ptr(), appkey.as_ptr(), secret.as_ptr())
            },
        })
    }

    pub fn execute<S: AsRef<str>>(&mut self,
                                  request: &mut TopRequest,
                                  session: Option<S>)
                                  -> Result<(Option<String>, TopResponseIterator), TopError> {
        let session_ptr;
        if let Some(s) = session {
            let session = CString::new(s.as_ref())?;
            session_ptr = session.into_raw();
        } else {
            session_ptr = ::std::ptr::null_mut();
        }

        unsafe {
            let response =
                TopResponse::from_raw(ffi::top_execute(self.ptr, request.ptr(), session_ptr));
            let session_ret = if session_ptr.is_null() {
                CString::from_raw(session_ptr);
                None
            } else {
                Some(CString::from_raw(session_ptr).into_string()?)
            };

            if response.code() == 0 {
                Ok((session_ret, TopResponseIterator::from_response(response)))
            } else {
                Err(TopError::Response(TopResponseError::extract_from_response(&response)))
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
        pub fn $name<S: AsRef<str>>(&mut self, key: S, value: S) -> Result<(), TopError> {
            let key = CString::new(key.as_ref())?;
            let value = CString::new(value.as_ref())?;
            unsafe {
                match ffi::$name(self.ptr, key.as_ptr(), value.as_ptr()) {
                    0 => Ok(()),
                    code => Err(TopError::from(code)),
                }
            }
        }
    }
}

impl TopRequest {
    pub fn new() -> TopRequest {
        TopRequest { ptr: unsafe { ffi::alloc_top_request() } }
    }

    pub fn set_api_name<S: AsRef<str>>(&mut self, name: S) -> Result<(), TopError> {
        let name = CString::new(name.as_ref())?;
        unsafe {
            match ffi::set_api_name(self.ptr, name.as_ptr()) {
                0 => Ok(()),
                code => Err(TopError::from(code)),
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

pub struct TopResponse<'a> {
    ptr: ffi::pTopResponse,
    marker: ::std::marker::PhantomData<&'a str>,
}

impl<'a> TopResponse<'a> {
    pub fn new() -> TopResponse<'a> {
        TopResponse {
            ptr: unsafe { ffi::alloc_top_response() },
            marker: ::std::marker::PhantomData,
        }
    }

    fn from_raw(raw: ffi::pTopResponse) -> TopResponse<'a> {
        TopResponse {
            ptr: raw,
            marker: ::std::marker::PhantomData,
        }
    }

    pub fn code(&self) -> i32 {
        unsafe { (*self.ptr).code as i32 }
    }

    pub fn sub_code(&self) -> &'a str {
        unsafe { CStr::from_ptr((*self.ptr).subCode).to_str().unwrap() }
    }

    pub fn sub_msg(&self) -> &'a str {
        unsafe { CStr::from_ptr((*self.ptr).subMsg).to_str().unwrap() }
    }

    fn ptr(&mut self) -> ffi::pTopResponse {
        self.ptr
    }
}

impl<'a> IntoIterator for TopResponse<'a> {
    type Item = Result<(&'a str, &'a str), TopError>;
    type IntoIter = TopResponseIterator<'a>;
    fn into_iter(self) -> TopResponseIterator<'a> {
        TopResponseIterator::from_response(self)
    }
}

impl<'a> Drop for TopResponse<'a> {
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
    fn from_response(mut response: TopResponse) -> TopResponseIterator<'a> {
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
    type Item = Result<(&'a str, &'a str), TopError>;

    fn next(&mut self) -> Option<Result<(&'a str, &'a str), TopError>> {
        unsafe {
            let resultitem = ffi::alloc_result_item();
            if ffi::parseNext(self.ptr, resultitem) == 0 {
                let resultitem = match ResultItem::from_raw(resultitem) {
                    Err(e) => return Some(Err(e)),
                    Ok(i) => i,
                };
                Some(Ok((resultitem.key, resultitem.value)))
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
    fn from_raw(raw: ffi::pResultItem) -> Result<ResultItem<'a>, TopError> {
        unsafe {
            let key = CStr::from_ptr((*raw).key).to_str()?;
            let value = CStr::from_ptr((*raw).value).to_str()?;
            Ok(ResultItem {
                key: key,
                value: value,
                ptr: raw,
            })
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

pub fn set_capath<P: AsRef<Path>>(path: P) -> Result<(), TopError> {
    let path = CString::new(path.as_ref().to_str().unwrap())?;
    let path_ptr = path.into_raw();
    unsafe {
        ffi::set_capath(path_ptr);
        CString::from_raw(path_ptr);
    }
    Ok(())
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
