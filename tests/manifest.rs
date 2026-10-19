use std::fs;

#[test]
fn manifest_queries_compile_and_parse_real_toml() {
    let language = tree_sitter_toml::language();
    let query = tree_sitter_020::Query::new(
        language,
        &fs::read_to_string("languages/openplanet_manifest/highlights.scm").unwrap(),
    )
    .unwrap();
    let mut parser = tree_sitter_020::Parser::new();
    parser.set_language(language).unwrap();
    let source = fs::read_to_string("fixtures/plugin/info.toml").unwrap();
    let tree = parser.parse(&source, None).unwrap();
    assert!(!tree.root_node().has_error());
    let mut cursor = tree_sitter_020::QueryCursor::new();
    assert!(
        cursor
            .captures(&query, tree.root_node(), source.as_bytes())
            .count()
            > 5
    );
}

#[test]
fn grammar_export_name_matches_upstream_parser() {
    let manifest: toml::Value =
        toml::from_str(&fs::read_to_string("extension.toml").unwrap()).unwrap();
    // Zed invokes tree_sitter_{grammar_name}; upstream exports tree_sitter_toml.
    assert!(manifest["grammars"].get("toml").is_some());
    let config: toml::Value =
        toml::from_str(&fs::read_to_string("languages/openplanet_manifest/config.toml").unwrap())
            .unwrap();
    assert_eq!(config["grammar"].as_str(), Some("toml"));
}
