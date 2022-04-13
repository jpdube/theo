use ncurses::*;

#[derive(Default)]
struct WindowManager {
    header: String,
    body: String,
    footer: String,
}

impl WindowManager {
    pub fn new() -> Self {
        WindowManager::default()
    }

    pub fn init(&self) {
        // println!("\u{2500}\u{2500}\u{2510}");
        // println!("  \u{2502}");

        initscr();
        raw();

        /* Allow for extended keyboard (like F1). */
        keypad(stdscr(), true);
        noecho();

        /* Invisible cursor. */
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        /* Status/help info. */
        addstr(&format!("Use the arrow keys to move, screen size: Lines:{}, Cols:{}", LINES(), COLS()));
        mvprintw(LINES() - 1, (COLS() / 2) - 8, "Press F1 to exit");
        refresh();

        let ch = getch();
        endwin();
    }
}

#[cfg(test)]
mod tests {
    use super::WindowManager;

    #[test]
    fn test_new() {
        let mut wm = WindowManager::new();
        wm.header = "Pcap db".to_owned();

        assert!(wm.header == "Pcap db".to_owned());
        wm.init();
    }
}
