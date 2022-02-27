use super::super::super::link::swap::{
    CountSwapOutput, CountSwapOutputDataElement, SwapPass, SwapPassSelection,
};
use crate::ImageDimensions;

struct SwapRatio {
    total: usize,
    accepted: CountSwapOutputDataElement,
}

impl SwapRatio {
    fn new(total: usize, accepted: CountSwapOutputDataElement) -> Self {
        assert!(accepted.is_finite());
        assert!(accepted >= 0.0);
        assert!(total > 0);
        assert!((accepted as usize) as CountSwapOutputDataElement == accepted);
        Self { total, accepted }
    }

    pub fn is_none_accepted(&self) -> bool {
        self.accepted == 0.0
    }

    pub fn accepted_fraction(&self) -> f64 {
        <CountSwapOutputDataElement as Into<f64>>::into(self.accepted) / self.total as f64
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn accepted(&self) -> usize {
        self.accepted as usize
    }
}

pub struct CountSwapOperationOutputPass {
    pass: SwapPass,
    swap_ratio: SwapRatio,
}

impl CountSwapOperationOutputPass {
    pub fn pass(&self) -> SwapPass {
        self.pass
    }

    pub fn is_none_accepted(&self) -> bool {
        self.swap_ratio.is_none_accepted()
    }

    pub fn accepted_fraction(&self) -> f64 {
        self.swap_ratio.accepted_fraction()
    }

    pub fn total(&self) -> usize {
        self.swap_ratio.total()
    }

    pub fn accepted(&self) -> usize {
        self.swap_ratio.accepted()
    }
}

pub struct CountSwapOperationOutput {
    passes: Vec<CountSwapOperationOutputPass>,
    combined_swap_ratio: SwapRatio,
}

impl CountSwapOperationOutput {
    pub(super) fn new(
        count_swap_output: &CountSwapOutput,
        selection: SwapPassSelection,
        image_dimensions: &ImageDimensions,
    ) -> Self {
        let (passes, total, accepted) = selection.iter().fold(
            (
                Vec::<CountSwapOperationOutputPass>::new(),
                0_usize,
                0 as CountSwapOutputDataElement,
            ),
            |mut acc, &pass| {
                let accepted_i = count_swap_output.at_pass(pass);
                let total_i = pass.total_swaps(image_dimensions);
                acc.0.push(CountSwapOperationOutputPass {
                    pass,
                    swap_ratio: SwapRatio::new(total_i, accepted_i),
                });
                acc.1 = acc.1.checked_add(total_i).unwrap();
                acc.2 += accepted_i;
                acc
            },
        );

        Self {
            passes,
            combined_swap_ratio: SwapRatio::new(total, accepted),
        }
    }

    pub fn passes(&self) -> &[CountSwapOperationOutputPass] {
        self.passes.as_slice()
    }

    pub fn is_none_accepted(&self) -> bool {
        self.combined_swap_ratio.is_none_accepted()
    }

    pub fn accepted_fraction(&self) -> f64 {
        self.combined_swap_ratio.accepted_fraction()
    }

    pub fn total(&self) -> usize {
        self.combined_swap_ratio.total()
    }

    pub fn accepted(&self) -> usize {
        self.combined_swap_ratio.accepted()
    }
}
