mod device;
mod dispatch;
mod operation;
mod output;
mod resource;

pub use dispatch::create_dispatcher;
pub use dispatch::CreatePermutationAlgorithm;
pub use dispatch::Dispatcher;
pub use output::Algorithm;
pub use output::OutputStatus;
