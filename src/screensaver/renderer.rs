use crate::screensaver::animations::AnimationPlayer;
use crate::ui::theme::Theme;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Paragraph},
};

/// Renders the current screensaver animation centered in the frame.
pub fn render_animation(frame: &mut Frame<'_>, player: &AnimationPlayer, theme: &Theme) {
    let area = frame.area();

    frame.render_widget(
        Block::default().style(Style::default().bg(theme.screensaver_bg)),
        area,
    );

    let anim_width = player.animation().width();
    let anim_height = player.animation().height();

    let x = (area.width.saturating_sub(anim_width)) / 2;
    let y = (area.height.saturating_sub(anim_height)) / 2;

    let center_area = Rect {
        x: area.x + x,
        y: area.y + y,
        width: anim_width,
        height: anim_height,
    };

    let paragraph = Paragraph::new(player.current_frame())
        .style(Style::default().fg(theme.screensaver_fg))
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, center_area);
}
