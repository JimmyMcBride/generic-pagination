use paginatator::Paged;

fn main() {
  let list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter();
  let thing = Paged::new(&list, 3).page(4);
  println!("THING: {:#?}", thing.unwrap().0);
}
