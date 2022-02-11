use std::io::{self, Write};

use termion::{color, style};

fn create_column_name() -> String {
    let mut row_names = format!("\n\t  {}|  ", color::Fg(color::Yellow));

    // for hex columns
    for i in 0..16 {
        row_names.push_str(&format!("{}{:02X} ", color::Fg(color::Magenta), i));
        if i % 4 == 3 {
            row_names.push(' ');
        }
    }

    // for character columns
    row_names.push_str(&format!("{}| ", color::Fg(color::Yellow)));
    for i in 0..16 {
        row_names.push_str(&format!("{}{:X}", color::Fg(color::Magenta), i));
    }

    // for print a line
    row_names.push_str(&format!(
        "{}\n{}|{}|{}",
        color::Fg(color::Yellow),
        "-".repeat(10),
        "-".repeat(54),
        "-".repeat(17)
    ));

    row_names
}

#[test]
fn test_create_column_name() {
    println!("{}", create_column_name());
}

fn print_hex_bytes(content: &[u8]) -> String {
    let mut sixteen_bytes = vec![];
    for i in (0..16).step_by(4) {
        let mut four_bytes = vec![];
        for j in i..i + 4 {
            let one_byte: String;
            if j >= content.len() {
                one_byte = "  ".to_owned();
            } else {
                let char_code = content[j];
                one_byte = match char_code {
                    32..=126 => format!("{}{:02X}", style::Reset, char_code),
                    9 | 10 | 13 => format!("{}{:02X}", color::Fg(color::Green), char_code),
                    _ => format!("{}{:02X}", color::Fg(color::Red), char_code),
                }
            }
            four_bytes.push(one_byte);
        }
        sixteen_bytes.push(four_bytes.join(" "));
    }
    sixteen_bytes.join("  ")
}

fn print_char_bytes(content: &[u8]) -> String {
    let mut res = String::new();
    content.iter().for_each(|byte| match *byte {
        32..=126 => res.push_str(&format!("{}{}", style::Reset, *byte as char)),
        9 | 10 | 13 => res.push_str(&format!("{}.", color::Fg(color::Green))),
        _ => res.push('.'),
    });
    res + &" ".repeat(16 - content.len())
}

fn create_rows(content: &[u8]) -> String {
    let mut rows = vec![];
    for i in (0..content.len()).step_by(16) {
        let row = format!(
            " {}{:08X} {}|  {}  {}| {}",
            color::Fg(color::Magenta),
            i,
            color::Fg(color::Yellow),
            print_hex_bytes(
                &content[i..(if i + 16 <= content.len() {
                    i + 16
                } else {
                    content.len()
                })]
            ),
            color::Fg(color::Yellow),
            print_char_bytes(
                &content[i..(if i + 16 <= content.len() {
                    i + 16
                } else {
                    content.len()
                })]
            ),
        );
        rows.push(row);
    }
    rows.join("\n")
}

#[test]
fn test_create_rows() {
    println!(
        "{}",
        create_rows(
            r#"
[package]
name = "hex-rs"
authors = ["fffzlfk <fffzlfk@qq.com>"]
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
log = "0.4.14"
clap = { version = "3.0.14", features = ["derive"] }
termion = "*""#
                .as_bytes()
        )
    );
}

pub fn display(content: &[u8]) -> std::io::Result<()> {
    io::stdout().write_all(create_column_name().as_bytes())?;
    io::stdout().write_all(b"\n")?;
    io::stdout().write_all(create_rows(content).as_bytes())?;
    Ok(())
}
