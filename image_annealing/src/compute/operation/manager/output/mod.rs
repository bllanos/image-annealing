use super::super::super::link::swap::{
    CountSwapOutput, CountSwapOutputDataElement, SwapPass, SwapPassSelection,
};
use crate::ImageDimensions;
use std::fmt;

struct SwapRatio {
    total: usize,
    accepted: CountSwapOutputDataElement,
}

impl SwapRatio {
    fn new(total: usize, accepted: CountSwapOutputDataElement) -> Self {
        assert!(
            accepted.is_finite(),
            "number of accepted swaps, {}, is not finite",
            accepted
        );
        assert!(
            accepted >= 0.0,
            "number of accepted swaps, {}, is negative",
            accepted
        );
        assert!(
            (accepted as usize) as CountSwapOutputDataElement == accepted,
            "number of accepted swaps, {}, is not an integer",
            accepted
        );
        assert!(
            accepted as usize <= total,
            "number of accepted swaps, {}, is greater than the total number of swaps, {}",
            accepted,
            total
        );
        Self { total, accepted }
    }

    pub fn is_none_accepted(&self) -> bool {
        self.accepted == 0.0
    }

    pub fn accepted_fraction(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            <CountSwapOutputDataElement as Into<f64>>::into(self.accepted) / self.total as f64
        }
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn accepted(&self) -> usize {
        self.accepted as usize
    }
}

impl fmt::Display for SwapRatio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} / {} ({:.2}%) swaps accepted",
            self.accepted(),
            self.total(),
            self.accepted_fraction() * 100.0
        )
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

impl fmt::Display for CountSwapOperationOutputPass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pass: {}, result: {}", self.pass, self.swap_ratio)
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

impl fmt::Display for CountSwapOperationOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "all passes: {}", self.combined_swap_ratio)?;
        for pass in self.passes.iter() {
            writeln!(f, "\t{}", pass)?;
        }
        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests;
