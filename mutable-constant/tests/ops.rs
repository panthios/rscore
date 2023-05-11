use mutable_constant::Mc;


#[test]
fn basic_add() {
  let mut mc = Mc::new(42);
  *mc = *mc + 1;

  assert_eq!(*mc, 43);
}