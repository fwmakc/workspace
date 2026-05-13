//! Text renderer: fontdue rasterisation + single wgpu pipeline.
//!
//! Glyph atlases are cached by (text, font_size). The expensive rasterization
//! and GPU texture upload only runs when text content changes. Vertex positions
//! (which depend on screen position and color) are rebuilt every frame from
//! cached glyph metrics — this is cheap arithmetic.

pub mod atlas;
mod renderer;

use bytemuck::{Pod, Zeroable};

pub use renderer::TextRenderer;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct TextVertex {
    pub(crate) position: [f32; 2],
    pub(crate) tex_coords: [f32; 2],
    pub(crate) color: [f32; 4],
}

/// Description of a single text block to render.
pub struct TextEntry<'a> {
    pub text: &'a str,
    pub font_size: f32,
    pub screen_x: f32,
    pub screen_y_baseline: f32,
    pub color: [f32; 4],
}

/// Word-wrap text into lines that fit within `max_width` pixels.
///
/// Returns a list of `(line_text, y_baseline_offset)` where the offset is
/// relative to the original baseline, increasing downward (each line is
/// `line_height` pixels lower).
pub fn wrap_text<'a>(
    text_renderer: &TextRenderer,
    text: &'a str,
    font_size: f32,
    max_width: f32,
    line_height: f32,
) -> Vec<(&'a str, f32)> {
    if text.is_empty() || max_width <= 0.0 {
        return vec![(text, 0.0)];
    }

    let full_width = text_renderer.measure_text_width(text, font_size);
    if full_width <= max_width {
        return vec![(text, 0.0)];
    }

    let mut lines = Vec::new();
    let mut remaining = text;
    let mut line_idx = 0;

    while !remaining.is_empty() {
        let (line, rest) = split_line(text_renderer, remaining, font_size, max_width);
        let y_offset = line_idx as f32 * line_height;
        lines.push((line, y_offset));
        remaining = rest;
        line_idx += 1;
    }

    lines
}

/// Split a single line at the last word boundary that fits within `max_width`.
fn split_line<'a>(
    text_renderer: &TextRenderer,
    text: &'a str,
    font_size: f32,
    max_width: f32,
) -> (&'a str, &'a str) {
    let mut best_break = 0usize;
    let mut width = 0.0f32;

    for (byte_idx, ch) in text.char_indices() {
        let advance = text_renderer.char_advance(ch, font_size);
        if width + advance > max_width && best_break > 0 {
            let (_, rest) = text.split_at(best_break);
            let line = &text[..best_break];
            return (line.trim_end(), rest);
        }
        width += advance;
        if ch.is_whitespace() {
            best_break = byte_idx + ch.len_utf8();
        }
    }

    (text, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_vertex_size_matches_layout() {
        let expected = std::mem::size_of::<[f32; 2]>()
            + std::mem::size_of::<[f32; 2]>()
            + std::mem::size_of::<[f32; 4]>();
        assert_eq!(std::mem::size_of::<TextVertex>(), expected);
    }

    #[test]
    fn text_vertex_is_32_bytes() {
        assert_eq!(std::mem::size_of::<TextVertex>(), 32);
    }
}
