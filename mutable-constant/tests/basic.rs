use mutable_constant::Mc;



#[test]
fn constant_access() {
  let mc = Mc::new(42);
  assert_eq!(*mc.as_ref(), 42);
}

#[test]
fn mutable_access() {
  let mut mc = Mc::new(42);

  *mc.as_mut() = 43;

  assert_eq!(*mc.as_ref(), 43);
}

#[test]
fn mutate_constant_ref() {
  let mc = Mc::new(42);
  let mc_ref = mc.as_ref();

  assert_eq!(*mc_ref, 42);

  unsafe {
    *mc.as_defiant_mut() = 43;
  }

  assert_eq!(*mc_ref, 43);
}