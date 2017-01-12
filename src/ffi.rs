// automatically generated by rust-bindgen

#[repr(C)]
#[derive(Debug, Copy)]
pub struct SingleMap {
    pub ite: ::std::os::raw::c_int,
    pub size: ::std::os::raw::c_int,
    pub total_length: ::std::os::raw::c_int,
    pub keys: *mut *mut ::std::os::raw::c_char,
    pub values: *mut *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_SingleMap() {
    assert_eq!(::std::mem::size_of::<SingleMap>(), 32usize);
    assert_eq!(::std::mem::align_of::<SingleMap>(), 8usize);
}
impl Clone for SingleMap {
    fn clone(&self) -> Self {
        *self
    }
}
pub type top_map = SingleMap;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct TopRequest {
    pub timestamp: ::std::os::raw::c_long,
    pub url: *mut ::std::os::raw::c_char,
    pub apiName: *mut ::std::os::raw::c_char,
    pub httpHeaders: *mut top_map,
    pub params: *mut top_map,
    pub files: *mut top_map,
}
#[test]
fn bindgen_test_layout_TopRequest() {
    assert_eq!(::std::mem::size_of::<TopRequest>(), 48usize);
    assert_eq!(::std::mem::align_of::<TopRequest>(), 8usize);
}
impl Clone for TopRequest {
    fn clone(&self) -> Self {
        *self
    }
}
pub type pTopRequest = *mut TopRequest;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct TopResponse {
    pub code: ::std::os::raw::c_int,
    pub msg: *mut ::std::os::raw::c_char,
    pub subCode: *mut ::std::os::raw::c_char,
    pub subMsg: *mut ::std::os::raw::c_char,
    pub requestId: *mut ::std::os::raw::c_char,
    pub results: *mut top_map,
    pub bytes: *mut ::std::os::raw::c_char,
    pub len: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_TopResponse() {
    assert_eq!(::std::mem::size_of::<TopResponse>(), 64usize);
    assert_eq!(::std::mem::align_of::<TopResponse>(), 8usize);
}
impl Clone for TopResponse {
    fn clone(&self) -> Self {
        *self
    }
}
pub type pTopResponse = *mut TopResponse;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ResultItem {
    pub key: *mut ::std::os::raw::c_char,
    pub value: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_ResultItem() {
    assert_eq!(::std::mem::size_of::<ResultItem>(), 16usize);
    assert_eq!(::std::mem::align_of::<ResultItem>(), 8usize);
}
impl Clone for ResultItem {
    fn clone(&self) -> Self {
        *self
    }
}
pub type pResultItem = *mut ResultItem;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct TopResponseIterator {
    pub cur_index: ::std::os::raw::c_int,
    pub pResult: pTopResponse,
}
#[test]
fn bindgen_test_layout_TopResponseIterator() {
    assert_eq!(::std::mem::size_of::<TopResponseIterator>(), 16usize);
    assert_eq!(::std::mem::align_of::<TopResponseIterator>(), 8usize);
}
impl Clone for TopResponseIterator {
    fn clone(&self) -> Self {
        *self
    }
}
pub type pTopResponseIterator = *mut TopResponseIterator;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct TaobaoClient {
    pub url: *mut ::std::os::raw::c_char,
    pub appkey: *mut ::std::os::raw::c_char,
    pub appsecret: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_TaobaoClient() {
    assert_eq!(::std::mem::size_of::<TaobaoClient>(), 24usize);
    assert_eq!(::std::mem::align_of::<TaobaoClient>(), 8usize);
}
impl Clone for TaobaoClient {
    fn clone(&self) -> Self {
        *self
    }
}
pub type pTaobaoClient = *mut TaobaoClient;
extern "C" {
    pub fn alloc_top_request() -> pTopRequest;
}
extern "C" {
    pub fn destroy_top_request(pt: pTopRequest);
}
extern "C" {
    pub fn add_httpheader_add_param(pt: pTopRequest,
                                    key: *const ::std::os::raw::c_char,
                                    value: *const ::std::os::raw::c_char)
                                    -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn add_param(pt: pTopRequest,
                     key: *const ::std::os::raw::c_char,
                     value: *const ::std::os::raw::c_char)
                     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn add_file_param(pt: pTopRequest,
                          key: *const ::std::os::raw::c_char,
                          value: *const ::std::os::raw::c_char)
                          -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn set_api_name(pt: pTopRequest,
                        name: *const ::std::os::raw::c_char)
                        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn alloc_top_response() -> pTopResponse;
}
extern "C" {
    pub fn destroy_top_response(pt: pTopResponse);
}
extern "C" {
    pub fn init_response_iterator(pResult: pTopResponse) -> pTopResponseIterator;
}
extern "C" {
    pub fn alloc_result_item() -> pResultItem;
}
extern "C" {
    pub fn parseNext(ite: pTopResponseIterator, resultItem: pResultItem) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn destroy_result_item(pResult: pResultItem);
}
extern "C" {
    pub fn destroy_response_iterator(ite: pTopResponseIterator);
}
extern "C" {
    pub fn destroy_taobao_client(pClient: pTaobaoClient);
}
extern "C" {
    pub fn alloc_taobao_client(url: *const ::std::os::raw::c_char,
                               appkey: *const ::std::os::raw::c_char,
                               secret: *const ::std::os::raw::c_char)
                               -> pTaobaoClient;
}
extern "C" {
    pub fn top_execute(pClient: pTaobaoClient,
                       request: pTopRequest,
                       session: *mut ::std::os::raw::c_char)
                       -> *mut TopResponse;
}
extern "C" {
    pub fn set_retry_times(retry: ::std::os::raw::c_int);
}
extern "C" {
    pub fn set_retry_sleep_times(sleep_time: ::std::os::raw::c_int);
}
extern "C" {
    pub fn set_http_time_out(timeout: ::std::os::raw::c_int);
}
extern "C" {
    pub fn set_capath(path: *mut ::std::os::raw::c_char);
}
