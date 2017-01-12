extern crate libbindgen;
extern crate gcc;

fn main() {
    // compile sdk
    gcc::compile_library("libtopsdk.a",
                         &["top-sdk/src/curlResponse.c",
                           "top-sdk/src/globalConfig.c",
                           "top-sdk/src/json.c",
                           "top-sdk/src/Md5Util.c",
                           "top-sdk/src/SingleMap.c",
                           "top-sdk/src/TaobaoClient.c",
                           "top-sdk/src/TopRequest.c",
                           "top-sdk/src/TopResponse.c",
                           "top-sdk/src/WebUtils.c"]);

    // create bindings
    let _ = libbindgen::builder()
        .header("top-sdk/topsdk.h")
        .use_core()
        .generate()
        .unwrap()
        .write_to_file("src/ffi.rs");
}
