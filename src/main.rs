use paginatator::Paged;

fn main() {
  let list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter();
  let current_page = 3;
  let paginated_date = Paged::new(&list, 3);
  let page = paginated_date.page(current_page).unwrap().0;
  println!(
    "Page {} of {}: {:#?}",
    current_page, paginated_date.total_pages, page
  );
}
