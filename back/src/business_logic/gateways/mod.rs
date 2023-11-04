pub trait RetrieveTreesQuery {
  fn get_all_trees() -> Result<Vec<TreeVM>>;
}