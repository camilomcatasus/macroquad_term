
#[cfg(not(target_arch="wasm32"))]
pub fn open_url(url: &str) {
    use opener::open;
    open(url).unwrap();
}
extern "C" {
    fn open_new_tab(ptr: *const i8, len: u32);
}
#[cfg(target_arch="wasm32")]
pub fn open_url(url: &str) {
    use std::ffi::CString;
    unsafe {
        let c_url = CString::new(url).unwrap();
        open_new_tab(c_url.as_ptr(), c_url.as_bytes().len() as u32);
    }
}
