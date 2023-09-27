pub struct RetrieveProjectTrees<Query: RetrieveTreesQuery> {
  retrieve_project_trees_query: Query
}

#[cfg(test)]
mod tests;