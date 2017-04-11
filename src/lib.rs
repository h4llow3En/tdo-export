//! The export module for Tdo server.
#![deny(missing_docs, unsafe_code,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]
#![warn(missing_debug_implementations)]
extern crate tdo_core;
extern crate libc;

use std::{slice, io, ptr};
use libc::{ioctl, c_ushort, STDOUT_FILENO, TIOCGWINSZ};

#[repr(C)]
struct winsize {
    ws_row: c_ushort, /* rows, in characters */
    ws_col: c_ushort, /* columns, in characters */
    ws_xpixel: c_ushort, /* horizontal size, pixels */
    ws_ypixel: c_ushort, /* vertical size, pixels */
}


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
        }
    }
}

/// Returns the formated output for the terminal printout.
#[allow(unused_variables, unused_mut)] //for now
pub fn render_terminal_output(tdo: &tdo_core::tdo::Tdo) -> Option<Vec<String>> {
    let (col, _): (usize, _) = match get_winsize() {
        Ok(res) => res,
        Err(_) => {
            println!("[Error] Terminalsize could not be fetched.", );
            std::process::exit(1);
        }
    };
    let mut formated_printout: Vec<String> = Vec::new();
    // TODO: Do the actual formating for the printout

    match formated_printout.len() {
        0 => None,
        _ => Some(formated_printout),
    }
}

#[allow(unsafe_code)]
fn get_full_name() -> Result<String, io::Error> {
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

#[allow(unsafe_code)]
fn get_winsize() -> io::Result<(usize, usize)> {
    let w = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) };

    match r {
        0 => Ok((w.ws_col as usize, w.ws_row as usize)),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "No valid data.")),
    }
}
