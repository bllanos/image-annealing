mod device;
mod dispatch;
mod link;
mod operation;
mod output;
mod resource;
mod system;

pub use dispatch::create_dispatcher;
pub use dispatch::Dispatcher;
pub use dispatch::{
    CreatePermutationAlgorithm, PermuteAlgorithm, SwapAlgorithm, ValidatePermutationAlgorithm,
};
pub use output::algorithm::create_permutation::{
    CreatePermutationInput, CreatePermutationParameters,
};
pub use output::algorithm::permute::{PermuteInput, PermuteOutput, PermuteParameters};
pub use output::algorithm::swap::{
    CountSwapOperationOutput, CountSwapOperationOutputPass, InvalidSwapParametersError,
    SwapFullOutput, SwapInput, SwapParameters, SwapPartialOutput, SwapPass, SwapPassSelection,
};
pub use output::algorithm::validate_permutation::{
    ValidatePermutationInput, ValidatePermutationParameters,
};
pub use output::conversion;
pub use output::format;
pub use output::Algorithm;
pub use output::OutputStatus;
pub use system::DimensionsMismatchError;
