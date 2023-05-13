use data_relations::OneToOne;


#[test]
fn basic() {
  let a = 1;
  let b = 2;

  let relation = OneToOne::new(&a, &b);

  assert_eq!(relation.from, &a);
  assert_eq!(relation.to, &b);
}