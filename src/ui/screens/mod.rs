pub mod dashboard;
pub mod finance;
pub mod management;
pub mod rental;
pub mod screensaver;

pub use dashboard::render_dashboard;
pub use finance::render_finance;
pub use management::render_management;
pub use rental::render_rental_management;
pub use screensaver::{create_screensaver_screen, render_screensaver};
