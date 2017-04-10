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

/// Generates a markdown String to export the lists.
pub fn gen_tasks_md(tdo: &tdo_core::tdo::Tdo, list_done: bool) -> Option<String> {
    let mut markdown = String::from("# Your tasks\n\n");
    let name = get_full_name();
    if name.is_ok() {
        markdown.push_str(&format!("Here are the tasks for {}\n\n", &name.unwrap()));
    }
    let mut intern = String::new();
    for list in tdo.to_owned().lists.iter() {
        let tasks: Vec<tdo_core::todo::Todo>;
        if list_done {
            tasks = list.list.to_owned();
        } else {
            tasks = list.list_undone();
        }
        if tasks.len() > 0 {
            intern.push_str(&format!("\n### {}\n", &list.name));
            for entry in tasks {
                if entry.done {
                    intern.push_str(&format!("- [x] {}\n", &entry.name));
                } else {
                    intern.push_str(&format!("- [ ] {}\n", &entry.name));
                }
            }
        }
    }
    match intern.len() {
        0 => None,
        _ => {
            markdown.push_str(&intern);
            Some(markdown)
        },
    }
}

/// Returns the full Name of the current user if present.
#[allow(unsafe_code)]
pub fn get_full_name() -> Result<String, io::Error> {
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
