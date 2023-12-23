#[test]
fn it_should_return_an_empty_vec_of_trees() {
    let retrieve_project_trees_query = InMemoryRetrieveProjectTreesQuery { trees: Vec::new() };
    let use_case = RetrieveProjectTrees {
        retrieve_project_trees_query,
    };
    let response = use_case.handle();
    assert_eq!(response, Vec::new());
}
