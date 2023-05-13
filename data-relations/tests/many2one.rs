use data_relations::ManyToOne;



#[test]
fn basic() {
  let a1 = 1;
  let a2 = 2;
  let b = 3;

  let relation = ManyToOne::new(vec![&a1, &a2], &b);

  assert_eq!(relation.from, vec![&a1, &a2]);
  assert_eq!(relation.to, &b);
}