mod admin;
mod map;
mod static_pages;

pub use admin::{AdminPage, AdminPageData};
pub use map::MapPage;
pub use static_pages::{AccountContext, AuthProviders, HomePage, HomePageData, LoginPage};
