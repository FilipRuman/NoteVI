use std::char;

use crossterm::style::Stylize;

pub fn get_style_for_line(line: String) -> crossterm::style::StyledContent<String> {
    let bytes = line.trim_matches(' ').as_bytes();
    let bytes_len = bytes.len();
    if bytes_len == 0 {
        return line.stylize();
    }
    let mut output = line.to_owned().stylize();

    match bytes[0] as char {
        '#' => {
            let mut hash_tag_count = 1;
            for i in 1..bytes_len {
                if bytes[i] as char == '#' {
                    hash_tag_count += 1;
                }
            }
            match hash_tag_count {
                1 => output = output.yellow(),
                2 => output = output.blue(),
                3 => output = output.cyan(),
                4 => output = output.green(),

                5 => output = output.grey(),
                _ => output = output.dark_grey(),
            }
            output = output.bold();
        }

        '-' => {
            output = output.bold().red();
        }
        '*' => {
            output = output.bold().red();
        }
        '+' => {
            output = output.bold().red();
        }
        _ => {}
    }

    return output;
}
