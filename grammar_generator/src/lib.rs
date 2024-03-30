mod constructor;
mod symbol_table;
fn count_lines(source: &String, start_position: u32) -> u32 {
    let mut new_line_count: u32 = 1;

    for i in &source.as_bytes()[0..start_position as usize] {
        if *i == b'\n' {
            new_line_count += 1;
        }
    }
    new_line_count
}
