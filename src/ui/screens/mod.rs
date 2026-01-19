pub mod dashboard;
pub mod finance;
pub mod management;
pub mod rental;

pub use dashboard::render_dashboard;
pub use finance::render_finance;
pub use management::render_management;
pub use rental::render_rental_management;
