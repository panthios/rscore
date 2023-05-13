use data_relations::ManyToMany;


#[test]
fn basic() {
  let a1 = 1;
  let a2 = 2;
  let b1 = 3;
  let b2 = 4;

  let relation = ManyToMany::new(vec![&a1, &a2], vec![&b1, &b2]);

  assert_eq!(relation.from, vec![&a1, &a2]);
  assert_eq!(relation.to, vec![&b1, &b2]);
}