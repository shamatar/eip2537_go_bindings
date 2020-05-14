extern "C" {
    pub fn c_eip2537_perform_operation(
        op: ::std::os::raw::c_char,
        i: *const ::std::os::raw::c_char,
        i_len: u32,
        o: *mut ::std::os::raw::c_char,
        o_len: *mut u32,
        err: *mut ::std::os::raw::c_char,
        char_len: *mut u32,
    ) -> u32;
}
