use super::TaggedPermutation;
use crate::config::PermutationPath;
use image_annealing::compute::format::ImageFileWriter;
use std::error::Error;
use std::future::Future;
use std::path::PathBuf;

pub struct TaggedPermutationWriter<'a> {
    path_prefix: &'a PermutationPath,
}

impl<'a> TaggedPermutationWriter<'a> {
    pub fn new(path_prefix: &'a PermutationPath) -> Self {
        Self { path_prefix }
    }

    pub fn save(
        &self,
        tagged_permutation: TaggedPermutation,
    ) -> impl Future<Output = Result<PathBuf, Box<dyn Error>>> + Unpin {
        let path_no_extension = format!(
            "{}_round_{}_pass_{}_{}",
            self.path_prefix,
            tagged_permutation.round_index,
            tagged_permutation.pass_index,
            tagged_permutation.pass.snake_case_name()
        );
        let result = tagged_permutation
            .permutation
            .save_add_extension(path_no_extension);
        std::future::ready(result)
    }
}
