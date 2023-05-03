pub trait Migration {
  fn up(db_client: DynDbClient);

  fn down(db_client: DynDbClient);
}


