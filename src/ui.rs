pub mod screens;
pub mod state;
pub mod theme;
pub mod widgets;

use crate::app::{App, AppScreen};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

/// Top-level render function that lays out the global UI regions.
pub fn render(frame: &mut Frame<'_>, app: &mut App) {
    let theme = app.theme();

    if app.screensaver_active() {
        app.screensaver_screen().render(frame, theme);
        return;
    }

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // header
            Constraint::Min(1),    // main content
            Constraint::Length(3), // keybind bar
            Constraint::Length(1), // status bar
        ])
        .split(frame.area());

    app.header_widget()
        .render(areas[0], frame.buffer_mut(), theme);

    match app.screen() {
        AppScreen::Dashboard => {
            if let Ok(data) = screens::dashboard::DashboardData::load(app.db(), app.settings()) {
                screens::dashboard::render(areas[1], frame.buffer_mut(), &data, theme);
            }
        }
        AppScreen::RentalManagement(tab) => {
            let workflow = app.active_workflow();
            let _ = screens::rental_management::render(
                areas[1],
                frame.buffer_mut(),
                tab,
                workflow,
                app.db(),
                theme,
            );
        }
        AppScreen::Finances => {
            let _ = screens::finances::render(areas[1], frame.buffer_mut(), app.db(), theme);
        }
        AppScreen::Management(tab) => {
            let _ = screens::management::render(
                areas[1],
                frame.buffer_mut(),
                tab,
                app.settings_editor(),
                app.db(),
                theme,
            );
        }
        AppScreen::Screensaver => {}
    }

    app.keybind_bar()
        .render(areas[2], frame.buffer_mut(), theme);
    app.status_bar().render(areas[3], frame.buffer_mut(), theme);
}
