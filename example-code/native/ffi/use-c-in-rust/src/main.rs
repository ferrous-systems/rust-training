use std::ffi::{c_char, c_uint}; // also in core::ffi

extern "C" {
    // We state that this function exists, but there's no definition.
    // The linker looks for this 'symbol name' in the other objects
    fn cool_library_function(p: *const c_char) -> c_uint;
}

fn main() {
    let s = c"123";
    let result: u32 = unsafe { cool_library_function(s.as_ptr()) };
    println!("cool_library_function({s:?}) => {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_integer() {
        let r = unsafe { cool_library_function(c"123".as_ptr()) };
        assert_eq!(r, 123);
    }

    #[test]
    fn invalid_integer() {
        let r = unsafe { cool_library_function(c"x123".as_ptr()) };
        assert_eq!(r, 0);
    }
}
