use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

use digitalization::{to_binary, to_decimal};


fn main() {
    // Initialize terminal and backend
    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Create a simple text widget
    let text = "Hello, world!";


    let widget = tui::widgets::Paragraph::new(text)
        .style(tui::style::Style::default().fg(tui::style::Color::White));

    // Draw the text widget to the terminal and refresh
    terminal.draw(|f| {
        let size = f.size();
        let x = (size.width - text.len() as u16) / 2;
        let y = size.height / 2;
        f.render_widget(widget, tui::layout::Rect::new(x, y, text.len() as u16, 1));
    }).unwrap();

    // Wait for user input
    for c in std::io::stdin().keys() {
        if let Ok(Key::Char('q')) = c {
            break;
        }
    }
}
