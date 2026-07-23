use _rammappy::align::Index;
#[test]
fn test_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Index>();
}
