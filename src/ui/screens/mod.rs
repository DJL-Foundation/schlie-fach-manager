pub mod dashboard;
pub mod finances;
pub mod management;
pub mod rental_management;
pub mod screensaver;

pub use dashboard::{DashboardScreen, format_currency};
pub use finances::FinancesScreen;
pub use management::ManagementScreen;
pub use rental_management::RentalManagementScreen;
pub use screensaver::ScreensaverScreen;
