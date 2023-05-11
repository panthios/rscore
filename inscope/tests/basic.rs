use inscope::Scope;


#[test]
fn no_special() {
  let x = Scope::new(5);
  assert_eq!(*x, Some(5));
}

#[test]
fn delete() {
  let x = Scope::new(5);
  assert_eq!(*x, Some(5));

  unsafe { x.delete() };
  assert_eq!(*x, None);
}