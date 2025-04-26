pub fn get_utf16_len(content: &str) -> u32 {
    content.chars().map(|c| c.len_utf16()).sum::<usize>() as u32
}
