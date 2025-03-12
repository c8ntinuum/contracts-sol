// programs/num-token/src/instructions/mod.rs
pub mod add_to_black_list;
pub mod generate;
pub mod initialize;
pub mod pause;
pub mod resume;
pub mod set_admin;
pub mod set_global_generation_price;
pub mod set_percentages;
pub mod set_verifier;
pub mod withdraw;

pub use add_to_black_list::*;
pub use generate::*;
pub use initialize::*;
pub use pause::*;
pub use resume::*;
pub use set_admin::*;
pub use set_global_generation_price::*;
pub use set_percentages::*;
pub use set_verifier::*;
pub use withdraw::*;
