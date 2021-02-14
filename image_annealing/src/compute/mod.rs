mod device;
mod dispatch;
mod operation;
mod output;
mod resource;

pub use dispatch::create_dispatcher;
pub use dispatch::CreatePermutationAlgorithm;
pub use dispatch::Dispatcher;
pub use output::conversion;
pub use output::create_permutation::{CreatePermutationInput, CreatePermutationParameters};
pub use output::format;
pub use output::Algorithm;
pub use output::OutputStatus;
