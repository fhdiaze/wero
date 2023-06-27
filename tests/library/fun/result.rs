use wero;

#[test]
pub fn test_map() {
  // Arrange
  let f = |x: i32| x * x;

  // Act
  let f_map = wero::infra::fun::result::map::<
    i32,
    wero::infra::core::result::AppResult<i32>,
    i32,
  >(&f);

  // Assert
  assert!(f_map(Ok(3)).is_ok());
}
