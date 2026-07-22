use std::fmt::Display;

pub enum SortingAlgorithm {
    Bubble,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortingStep {
    Swap(usize, usize),
    Compare(usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwapStep {
    None,
    Highlight,
    Swap,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompareStep {
    None,
    True,
    False,
}

#[derive(Default)]
pub struct SortingTrace {
    trace: Vec<SortingStep>,
    index: usize,
}

impl SortingTrace {
    pub fn new<T, F>(array: &[T], method: F) -> Self
    where
        T: PartialOrd + Clone,
        F: FnOnce(&[T]) -> Vec<SortingStep>,
    {
        Self {
            trace: method(array),
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.trace.len()
    }

    pub fn next(&mut self) -> Option<SortingStep> {
        let step = self.trace.get(self.index).copied();
        if self.index <= self.trace.len() {
            self.index = self.index.saturating_add(1);
        }

        step
    }

    pub fn is_next_swap(&self) -> bool {
        self.trace
            .get(self.index)
            .is_some_and(|step| matches!(step, SortingStep::Swap(_, _)))
    }

    pub fn is_next_compare(&self) -> bool {
        self.trace
            .get(self.index)
            .is_some_and(|step| matches!(step, SortingStep::Compare(_, _)))
    }
}

impl From<u8> for SortingAlgorithm {
    fn from(value: u8) -> Self {
        match value {
            0 => SortingAlgorithm::Bubble,
            _ => SortingAlgorithm::Other,
        }
    }
}

impl From<SortingAlgorithm> for Vec<&'static str> {
    fn from(_: SortingAlgorithm) -> Self {
        vec!["Bubble", "Other"]
    }
}

impl Display for SortingTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.trace)
    }
}

pub fn bubble_sort<T>(array: &[T]) -> Vec<SortingStep>
where
    T: PartialOrd + Clone,
{
    let mut trace = Vec::new();
    let mut vec = array.to_vec();

    for i in 0..=vec.len() - 2 {
        for j in 0..=vec.len() - i - 2 {
            trace.push(SortingStep::Compare(j, j + 1));
            if vec[j] > vec[j + 1] {
                trace.push(SortingStep::Swap(j, j + 1));
                vec.swap(j, j + 1);
            }
        }
    }

    trace
}

pub fn quick_sort<T>(array: &[T]) -> Vec<SortingStep>
where
    T: PartialOrd + Clone,
{
    let mut trace = Vec::new();
    let mut vec = array.to_vec();

    trace
}

pub fn insertion_sort<T>(array: &[T]) -> Vec<SortingStep>
where
    T: PartialOrd + Clone,
{
    let mut trace = Vec::new();
    let mut vec = array.to_vec();

    trace
}

pub fn selection_sort<T>(array: &[T]) -> Vec<SortingStep>
where
    T: PartialOrd + Clone,
{
    let mut trace = Vec::new();
    let mut vec = array.to_vec();

    trace
}

pub fn heap_sort<T>(array: &[T]) -> Vec<SortingStep>
where
    T: PartialOrd + Clone,
{
    let mut trace = Vec::new();
    let mut vec = array.to_vec();

    trace
}

pub fn merge_sort<T>(array: &[T]) -> Vec<SortingStep>
where
    T: PartialOrd + Clone,
{
    let mut trace = Vec::new();
    let mut vec = array.to_vec();

    trace
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    fn build_vec(n: usize, min: u16, max: u16) -> Vec<u16> {
        (0..n).map(|_| rand::random_range(min..max)).collect()
    }

    fn apply_steps<T: Clone>(initial: &[T], steps: &Vec<SortingStep>) -> Vec<T> {
        let mut result = initial.to_vec();
        for step in steps {
            match step {
                SortingStep::Swap(i, j) => {
                    assert!(*i < result.len() && *j < result.len());
                    result.swap(*i, *j);
                }
                SortingStep::Compare(i, j) => {
                    assert!(*i < result.len() && *j < result.len());
                }
            }
        }

        result
    }

    fn assert_sort_correction<T: Ord + Clone + std::fmt::Debug>(
        initial: &[T],
        method: impl Fn(&[T]) -> Vec<SortingStep>,
    ) {
        let steps = method(initial);
        let mut expected = initial.to_vec();
        expected.sort();

        let actual = apply_steps(initial, &steps);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bubble_sort() {
        let vec = build_vec(30, 0, 100);
        assert_sort_correction(vec.as_slice(), bubble_sort);
    }

    #[test]
    fn test_quick_sort() {
        let vec = build_vec(30, 0, 100);
        assert_sort_correction(vec.as_slice(), quick_sort);
    }

    #[test]
    fn test_insertion_sort() {
        let vec = build_vec(30, 0, 100);
        assert_sort_correction(vec.as_slice(), insertion_sort);
    }

    #[test]
    fn test_selection_sort() {
        let vec = build_vec(30, 0, 100);
        assert_sort_correction(vec.as_slice(), selection_sort);
    }

    #[test]
    fn test_heap_sort() {
        let vec = build_vec(30, 0, 100);
        assert_sort_correction(vec.as_slice(), heap_sort);
    }

    #[test]
    fn test_merge_sort() {
        let vec = build_vec(30, 0, 100);
        assert_sort_correction(vec.as_slice(), merge_sort);
    }
}
