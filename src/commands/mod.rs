pub mod add;
pub mod diff;
pub mod status;

pub use add::{run_add_app, run_add_dependency, run_add_dotfiles, save_and_offer_gist_update, update_gist};
pub use diff::run_diff;
pub use status::run_status;
