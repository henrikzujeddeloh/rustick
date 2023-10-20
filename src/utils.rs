use terminal_size::{Width, Height, terminal_size};

pub fn get_term_width() -> u16 {
    if let Some((Width(w), Height(h))) = terminal_size() {
        return w;
    } else {
        println!("Unable to get terminal size");
        return 0;
    }
}
