use super::super::super::link::swap::{
    CountSwapOutput, CountSwapOutputDataElement, SwapPass, SwapPassSelection,
};
use super::super::super::output::algorithm::swap::{
    SwapPassSelectionSwapRatio, SwapPassSwapRatio, SwapRatio,
};
use crate::ImageDimensions;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
struct SwapRatioImplementation {
    total: usize,
    accepted: CountSwapOutputDataElement,
}

impl Eq for SwapRatioImplementation {}

impl SwapRatioImplementation {
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
}

impl SwapRatio for SwapRatioImplementation {
    fn accepted_fraction(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            <CountSwapOutputDataElement as Into<f64>>::into(self.accepted) / self.total as f64
        }
    }

    fn total(&self) -> usize {
        self.total
    }

    fn accepted(&self) -> usize {
        self.accepted as usize
    }
}

impl fmt::Display for SwapRatioImplementation {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CountSwapOperationOutputPass {
    pass: SwapPass,
    swap_ratio: SwapRatioImplementation,
}

impl SwapRatio for CountSwapOperationOutputPass {
    fn accepted_fraction(&self) -> f64 {
        self.swap_ratio.accepted_fraction()
    }

    fn total(&self) -> usize {
        self.swap_ratio.total()
    }

    fn accepted(&self) -> usize {
        self.swap_ratio.accepted()
    }
}

impl SwapPassSwapRatio for CountSwapOperationOutputPass {
    fn pass(&self) -> SwapPass {
        self.pass
    }
}

impl fmt::Display for CountSwapOperationOutputPass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pass: {}, result: {}", self.pass, self.swap_ratio)
    }
}

struct CountSwapOperationOutputPassIter<'a> {
    iter: std::slice::Iter<'a, CountSwapOperationOutputPass>,
}

impl<'a> Iterator for CountSwapOperationOutputPassIter<'a> {
    type Item = &'a dyn SwapPassSwapRatio;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item as &dyn SwapPassSwapRatio)
    }
}

pub struct CountSwapOperationOutput {
    passes: Vec<CountSwapOperationOutputPass>,
    combined_swap_ratio: SwapRatioImplementation,
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
                    swap_ratio: SwapRatioImplementation::new(total_i, accepted_i),
                });
                acc.1 = acc.1.checked_add(total_i).unwrap();
                acc.2 += accepted_i;
                acc
            },
        );

        Self {
            passes,
            combined_swap_ratio: SwapRatioImplementation::new(total, accepted),
        }
    }
}

impl SwapRatio for CountSwapOperationOutput {
    fn accepted_fraction(&self) -> f64 {
        self.combined_swap_ratio.accepted_fraction()
    }

    fn total(&self) -> usize {
        self.combined_swap_ratio.total()
    }

    fn accepted(&self) -> usize {
        self.combined_swap_ratio.accepted()
    }
}

impl SwapPassSelectionSwapRatio for CountSwapOperationOutput {
    fn passes<'a, 'b>(&'a self) -> Box<dyn Iterator<Item = &'a dyn SwapPassSwapRatio> + 'b>
    where
        'a: 'b,
    {
        Box::new(CountSwapOperationOutputPassIter {
            iter: self.passes.iter(),
        })
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
