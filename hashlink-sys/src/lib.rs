#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CString, c_void};
//use std::fs;
use std::fs::File;
use std::io::Read;

mod ffi;

pub use ffi::*;

pub fn test_main() {
    unsafe {
        let argv: [CString; 0] = [];
        let hlfile = CString::new("").expect("could not convert file contents to CString");

        hl_global_init();

        /*
         * extern "C" {
                  pub fn hl_sys_init(
                      args: *mut *mut ::std::os::raw::c_void,
                      nargs: ::std::os::raw::c_int,
                      hlfile: *mut ::std::os::raw::c_void,
                  );
              }
        */
        hl_sys_init(argv.as_ptr() as *mut *mut c_void, argv.len() as i32, hlfile.as_ptr() as *mut c_void);


        //let fdata = CString::new("").unwrap();


        let mut file = File::open("../../haxe/hello.hl").unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).unwrap();
        //let fdata = CString::new(buf).unwrap();
        let fdata = &buf[..];

        let error_msg = CString::new("").unwrap();
        /*
         * extern "C" {
                    pub fn hl_code_read(
                        data: *const ::std::os::raw::c_uchar,
                        size: ::std::os::raw::c_int,
                        error_msg: *mut *mut ::std::os::raw::c_char,
                    ) -> *mut hl_code;
                }
        */
        //let code = hl_code_read(fdata.as_ptr() as *const u8, fdata.to_bytes().len() as i32, error_msg.as_ptr() as *mut *mut i8);
        let code = hl_code_read(fdata.as_ptr() as *const u8, fdata.len() as i32, error_msg.as_ptr() as *mut *mut i8);
        println!("code ok");
        //let module = hl_module_alloc(code);
        // dbg!(code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            floorf(5.5);
        }
    }
}
