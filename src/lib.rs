//! The export module for Tdo server.
#![deny(missing_docs, unsafe_code,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]
#![warn(missing_debug_implementations)]
extern crate tdo_core;
extern crate libc;
extern crate colored;

#[macro_use]
extern crate prettytable;
use prettytable::Table;
use prettytable::format;
// use prettytable::row::Row;
// use prettytable::cell::Cell;

use std::{slice, io, ptr};
use colored::*;

#[repr(C)]
struct winsize {
    ws_row: libc::c_ushort, /* rows, in characters */
    ws_col: libc::c_ushort, /* columns, in characters */
    ws_xpixel: libc::c_ushort, /* horizontal size, pixels */
    ws_ypixel: libc::c_ushort, /* vertical size, pixels */
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
        } else if list_done {
            intern.push_str(&format!("\n### {}\n", &list.name));
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
pub fn render_terminal_output(tdo: &tdo_core::tdo::Tdo, all: bool) {
    let (width, _) = match get_winsize() {
        Ok(x) => x,
        Err(_) => {
            println!("{} Terminalsize could not be fetched.",
                     "error:".red().bold());
            std::process::exit(1);
        }
    };
    let mut table = Table::new();
    for list in tdo.lists.to_owned().iter() {
        let tasks: Vec<tdo_core::todo::Todo>;
        if all {
            tasks = list.list.to_owned();
        } else {
            tasks = list.list_undone();
        }
        table.add_row(row![bc->"###", "", b->&list.name]);
        if tasks.len() > 0 {
            for entry in tasks {
                let mut task_vec = reformat_task(&entry.name,
                                                 width - 9 -
                                                 tdo.get_highest_id().to_string().len());
                if entry.done {
                    table.add_row(row![c->"[x]", r->entry.id, task_vec.remove(0)]);
                } else {
                    table.add_row(row![c->"[ ]", r->entry.id, task_vec.remove(0)]);
                }
                if task_vec.capacity() > 0 {
                    for part in task_vec {
                        table.add_row(row!["", "", part]);
                    }
                }
            }
        }
        table.add_row(row![""]);
    }
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.printstd();
}


#[allow(unsafe_code)]
fn get_full_name() -> Result<String, io::Error> {
    unsafe {
        let uid = libc::geteuid();
        let user = ptr::read(libc::getpwuid(uid));
        let res = String::from_utf8_unchecked(slice::from_raw_parts(user.pw_gecos as *const u8,
                                                                    libc::strlen(user.pw_gecos))
            .to_vec());
        let results: Vec<&str> = res.split(",").collect();
        let name = results[0].to_string();
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
    let r = unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &w) };

    match r {
        0 => Ok((w.ws_col as usize, w.ws_row as usize)),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "No valid data.")),
    }
}

fn reformat_task(task_str: &str, size: usize) -> Vec<String> {
    let mut task_vec: Vec<String> = Vec::new();
    let mut temp_vec: Vec<&str> = task_str.split_whitespace().collect();
    while temp_vec.len() > 0 {
        let mut temp_str = String::new();
        while temp_str.len() + temp_vec[0].len() < size {
            temp_str.push_str(temp_vec.remove(0));
            temp_str.push_str(" ");
            if temp_vec.len() == 0 {
                task_vec.push(temp_str);
                return task_vec;
            }
        }
        task_vec.push(temp_str);
    }
    task_vec
}