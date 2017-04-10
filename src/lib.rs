//! The export module for Tdo server.
#![deny(missing_docs, unsafe_code,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]
#![warn(missing_debug_implementations)]
extern crate tdo_core;
extern crate libc;

use std::{slice, io, ptr};

/// Generates a well formated String of all undone Todos
pub fn gen_tasks_mail(tdo: &tdo_core::tdo::Tdo) -> Option<String> {
    let mut listed = String::new();
    for list in tdo.to_owned().lists.into_iter() {
        let undone = list.list_undone();
        if undone.len() > 0 {
            listed.push_str("\n------------------------------------------------------------\n\t");
            listed.push_str(&list.name);
            listed.push_str("\n------------------------------------------------------------\n");
            for entry in undone {
                if entry.done {
                    listed.push_str(&format!("- {:?}\n", entry.name));
                }
            }
            listed.push_str("\n\n");
        }
    }
    match listed.len() {
        0 => None,
        _ => Some(listed),
    }
}

/// Returns the full Name of the current user if present.
#[allow(unsafe_code)]
pub fn get_full_name() -> Result<String, io::Error>{
    unsafe {
        let uid = libc::geteuid();
        let user = ptr::read(libc::getpwuid(uid));
        let name = String::from_utf8_unchecked(slice::from_raw_parts(user.pw_gecos as *const u8,
                                                                     libc::strlen(user.pw_gecos))
            .to_vec());
        if name == "" {
            Err(io::Error::last_os_error())
        } else {
            Ok(name)
        }
    }
}
