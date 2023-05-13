use data_relations::OneToMany;


#[test]
fn basic() {
  let a = 1;
  let b1 = 2;
  let b2 = 3;

  let relation = OneToMany::new(&a, vec![&b1, &b2]);

  assert_eq!(relation.from, &a);
  assert_eq!(relation.to, vec![&b1, &b2]);
}