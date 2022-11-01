use fltk::{enums::*, prelude::*, *};
use fltk_richtext::{RichTextBuilder, RichTextDisplay, Style};
use regex::Regex;

const MARKUP: &str = r#"# The important points are:
- Point 1
- Point 2
- Point 3
"#;

fn handle_markup(s: &str) -> RichTextBuilder {
    let re = Regex::new(r"(^# .*)").unwrap();
    let mut buf = RichTextBuilder::new();
    buf.append(s, None);
    for caps in re.captures_iter(s) {
        buf.replace_all(
            caps.get(1).unwrap().as_str(),
            &caps.get(1).unwrap().as_str()[2..],
            Style {
                font: Font::HelveticaBold,
                size: app::font_size() + 2,
                ..Default::default()
            },
        );
    }
    buf.replace_all(
        "-",
        "  •",
        Style {
            color: Color::Red,
            ..Default::default()
        },
    );
    buf
}

fn main() {
    let a = app::App::default();
    let mut w = window::Window::default().with_size(400, 300);
    let mut disp = text::TextDisplay::default_fill();
    w.end();
    w.show();

    disp.set_rich_text(handle_markup(MARKUP));

    a.run().unwrap();
}
