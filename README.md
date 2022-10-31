# fltk-richtext

A small crate providing simpler RichText formatting for fltk-rs.

```rust
use fltk::{enums::*, prelude::*, *};
use fltk_richtext::{RichTextBuilder, RichTextDisplay, Style};

fn main() {
    let a = app::App::default();
    let mut w = window::Window::default().with_size(400, 300);
    let mut disp = text::TextDisplay::default_fill();
    w.end();
    w.show();

    let mut buf = RichTextBuilder::new();
    buf.append("Lorem ipsum text", None);
    buf.append(
        "::\n",
        Style {
            color: Color::DarkYellow,
            ..Default::default()
        },
    );
    buf.append(
        "Lorem ipsum, dolor sit amet\n",
        Style {
            color: Color::Red,
            font: Font::Times,
            ..Default::default()
        },
    );
    buf.append(
        " consectetur adipisicing elit.\n",
        Style {
            color: Color::Blue,
            ..Default::default()
        },
    );
    buf.append(
        "Aliquam expedita odio dignissimos\n",
        Style {
            color: Color::Green.darker(),
            ..Default::default()
        },
    );
    buf.append(
        "ducimus perspiciatis dolorum,\n",
        Style {
            font: Font::Courier,
            size: 14,
            ..Default::default()
        },
    );
    buf.append(
        "itaque dolorem similique velit doloribus,\n",
        Style {
            color: Color::White,
            bgcolor: Color::Magenta,
            attr: text::TextAttr::BgColor,
            ..Default::default()
        },
    );
    buf.append(
        "debitis ea ex necessitatibus quia,\n",
        Style {
            attr: text::TextAttr::Grammar,
            ..Default::default()
        },
    );
    buf.append(
        "optio maiores a voluptates. Animi.\n",
        Style {
            attr: text::TextAttr::Spelling,
            ..Default::default()
        },
    );
    disp.set_rich_text(buf);
    
    a.run().unwrap();
}
```

![image](https://user-images.githubusercontent.com/37966791/199112924-58faa1a9-032e-438e-8122-2849c146b49e.png)