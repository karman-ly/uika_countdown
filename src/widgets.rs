use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

pub struct TextInput {}

impl Widget for TextInput {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}

pub struct DateInput {}

impl Widget for DateInput {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
