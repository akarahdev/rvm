pub struct Terminal;

impl Terminal {
    pub fn clear_screen() {
        print!("{}[2J", 27 as char);
    }

    pub fn reset_screen() {
        Terminal::clear_screen();
        Terminal::move_cursor_to(1, 1);
    }

    pub fn move_cursor_to(row: usize, column: usize) {
        print!("{esc}[2J{esc}[{row};{column}H", esc = 27 as char);
    }
}
