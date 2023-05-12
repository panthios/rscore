use moment_node::Moment;


fn assert_moment<T: Moment>() {}


#[tokio::test]
async fn moment_compatible() {
  assert_moment::<u8>();
  assert_moment::<i16>();
  assert_moment::<f32>();
  assert_moment::<bool>();
  assert_moment::<char>();

  assert_moment::<Option<u8>>();
}