use math::round;
use std::{cmp::min, marker::PhantomData};

#[derive(Debug)]
pub struct Paged<'a, T, V> {
  vec: &'a V,
  results_per_page: usize,
  pub total_pages: usize,
  phantom: PhantomData<&'a T>,
}

impl<'a, T, V> Paged<'a, T, V>
where
  V: AsRef<[T]> + ExactSizeIterator,
{
  pub fn new(vec: &'a V, results_per_page: usize) -> Paged<'a, T, V> {
    Paged {
      vec,
      results_per_page,
      total_pages: round::ceil(vec.len() as f64 / results_per_page as f64, 0) as usize,
      phantom: PhantomData,
    }
  }

  pub fn page(&self, page_number: usize) -> Option<(&'a [T], usize)> {
    let index = self.results_per_page * page_number - self.results_per_page;
    let slice = self.vec.as_ref();
    let len = slice.len();

    if index < len {
      let page_index = index % self.results_per_page;
      let start = index - page_index;
      let end = min(len, start + self.results_per_page);

      slice.get(start..end).map(|s| (s, page_number))
    } else {
      None
    }
  }
}
