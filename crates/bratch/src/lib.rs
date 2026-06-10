pub mod cli;
pub mod compare;
pub mod error;
pub mod substrate_input;
pub mod taxonomy;
pub mod verdict_class;

pub use cli::{BratchCli, Command, CompareArgs};
pub use error::BratchError;
pub use substrate_input::RevisionIdPair;
pub use taxonomy::RegressionSignature;
pub use verdict_class::VerdictClass;

pub fn routing_key() -> bsuite_core::RoutingKey {
    bsuite_core::RoutingKey::bratch()
}
