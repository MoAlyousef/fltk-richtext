#![doc = include_str!("../README.md")]

use fltk::{enums::*, prelude::*, *};
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone, Debug)]
pub struct Style {
    pub color: Color,
    pub font: Font,
    pub size: i32,
    pub attr: text::TextAttr,
    pub bgcolor: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            color: Color::Foreground,
            font: Font::Helvetica,
            size: app::font_size(),
            attr: text::TextAttr::None,
            bgcolor: Color::Background2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RichTextBuilder {
    buf: text::TextBuffer,
    sbuf: text::TextBuffer,
    data: Arc<Mutex<Vec<text::StyleTableEntryExt>>>,
}

impl Default for RichTextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RichTextBuilder {
    pub fn new() -> Self {
        let buf = text::TextBuffer::default();
        let sbuf = text::TextBuffer::default();
        let style = Style::default();
        let data = Arc::new(Mutex::new(vec![text::StyleTableEntryExt {
            color: style.color,
            font: style.font,
            size: style.size,
            attr: style.attr,
            bgcolor: style.bgcolor,
        }]));
        Self { buf, sbuf, data }
    }
    pub fn append<T: Into<Option<Style>>>(&mut self, txt: &str, style: T) {
        self.buf.append(txt);
        if let Some(style) = style.into() {
            let mut data = self.data.lock().unwrap();
            let se = text::StyleTableEntryExt {
                color: style.color,
                font: style.font,
                size: style.size,
                attr: style.attr,
                bgcolor: style.bgcolor,
            };
            let idx = data.iter().position(|&i| i == se).unwrap_or((*data).len());
            self.sbuf
                .append(&((b'A' + idx as u8) as char).to_string().repeat(txt.len()));
            if idx == (*data).len() {
                (*data).push(se);
            }
        } else {
            self.sbuf
                .append(&(b'A' as char).to_string().repeat(txt.len()));
        }
    }
    pub fn clear(&mut self) {
        self.buf.set_text("");
        self.sbuf.set_text("");
        self.data.lock().unwrap().clear();
    }
    pub fn replace_first<T: Into<Option<Style>>>(&mut self, old: &str, new: &str, style: T) {
        let mut buf = self.buf.text();
        let mut sbuf = self.sbuf.text();
        if let Some(find) = buf.find(old) {
            if let Some(style) = style.into() {
                let mut data = self.data.lock().unwrap();
                let se = text::StyleTableEntryExt {
                    color: style.color,
                    font: style.font,
                    size: style.size,
                    attr: style.attr,
                    bgcolor: style.bgcolor,
                };
                let idx = data.iter().position(|&i| i == se).unwrap_or((*data).len());
                let range = find..find + old.len();
                sbuf.replace_range(
                    range.clone(),
                    &((b'A' + idx as u8) as char).to_string().repeat(new.len()),
                );
                buf.replace_range(range, new);
                if idx == (*data).len() {
                    (*data).push(se);
                }
            } else {
                let range = find..find + old.len();
                sbuf.replace_range(range.clone(), &(b'A' as char).to_string().repeat(new.len()));
                buf.replace_range(range, new);
            }
        }
        self.buf.set_text(&buf);
        self.sbuf.set_text(&sbuf);
    }
    pub fn replace_all<T: Into<Option<Style>> + Clone>(&mut self, old: &str, new: &str, style: T) {
        let mut idx = 0;
        while let Some(find) = &self.buf.text()[idx..].find(old) {
            self.replace_first(old, new, style.clone());
            idx = *find + 1;
        }
    }
}

pub trait RichTextDisplay {
    fn set_rich_text(&mut self, buf: RichTextBuilder);
}

impl<T: DisplayExt> RichTextDisplay for T {
    fn set_rich_text(&mut self, buf: RichTextBuilder) {
        self.set_buffer(buf.buf);
        self.set_highlight_data_ext(buf.sbuf, (*buf.data.lock().unwrap()).clone());
    }
}
