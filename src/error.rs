use super::TopResponse;

#[derive(Debug)]
pub struct TopResponseError {
    code: i32,
    sub_code: String,
    sub_msg: String,
}

impl TopResponseError {
    pub fn extract_from_response<'a>(response: &TopResponse) -> TopResponseError {
        TopResponseError {
            code: response.code(),
            sub_code: response.sub_code().to_string(),
            sub_msg: response.sub_msg().to_string(),
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum TopError {
        /// ffi pointer casting error
        StringConversion(err: ::std::ffi::NulError) {
            from(err: ::std::ffi::NulError) -> (err)
        }
        /// UTF8 error
        Utf8Error(err: ::std::str::Utf8Error) {
            from(err: ::std::str::Utf8Error) -> (err)
        }
        /// bad response
        Response(err: TopResponseError) {
            from(err: TopResponseError) -> (err)
        }
        /// bad parameter
        Parameter(err: isize) {
            from(err: ::std::os::raw::c_int) -> (err as isize)
            display("Bad parameter, error code {}", err)
        }
    }
}
