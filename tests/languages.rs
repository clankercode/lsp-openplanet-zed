use std::{fs, path::Path};
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

#[test]
fn all_angelscript_queries_compile_against_pinned_grammar() {
    let language = tree_sitter_angelscript::LANGUAGE.into();
    for entry in fs::read_dir("languages/angelscript").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|extension| extension == "scm") {
            Query::new(&language, &fs::read_to_string(&path).unwrap())
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        }
    }
}

#[test]
fn generalized_angelscript_syntax_and_plugin_parse_without_errors() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_angelscript::LANGUAGE.into())
        .unwrap();
    for path in ["fixtures/syntax.as", "fixtures/plugin/Main.as"] {
        let source = fs::read_to_string(path).unwrap();
        let tree = parser.parse(&source, None).unwrap();
        assert!(
            !tree.root_node().has_error(),
            "{path}: {}",
            tree.root_node().to_sexp()
        );
    }
    for ty in ["string", "const string", "Node@", "const Node@"] {
        for direction in ["in", "out", "inout"] {
            let source = format!("void F({ty} &{direction} value) {{}}\n");
            let tree = parser.parse(&source, None).unwrap();
            assert!(
                !tree.root_node().has_error(),
                "{source}: {}",
                tree.root_node().to_sexp()
            );
        }
    }
}

#[test]
fn highlight_and_outline_queries_produce_semantic_captures() {
    let language = tree_sitter_angelscript::LANGUAGE.into();
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    let source = fs::read_to_string("fixtures/syntax.as").unwrap();
    let tree = parser.parse(&source, None).unwrap();
    for (name, expected) in [
        ("highlights", vec!["Counter", "Add", "uint"]),
        ("outline", vec!["Example", "Counter", "Mode", "Visit"]),
    ] {
        let query = Query::new(
            &language,
            &fs::read_to_string(format!("languages/angelscript/{name}.scm")).unwrap(),
        )
        .unwrap();
        let mut cursor = QueryCursor::new();
        let mut captures = cursor.captures(&query, tree.root_node(), source.as_bytes());
        let mut captured = Vec::new();
        while let Some((matched, index)) = captures.next() {
            captured.push(
                matched.captures[*index]
                    .node
                    .utf8_text(source.as_bytes())
                    .unwrap()
                    .to_owned(),
            );
        }
        for text in expected {
            assert!(
                captured.iter().any(|c| c == text),
                "{name}: missing {text} in {captured:?}"
            );
        }
    }
}

#[test]
fn extension_and_language_contracts_are_consistent() {
    let manifest: toml::Value =
        toml::from_str(&fs::read_to_string("extension.toml").unwrap()).unwrap();
    let cargo: toml::Value = toml::from_str(&fs::read_to_string("Cargo.toml").unwrap()).unwrap();
    assert_eq!(
        manifest["version"].as_str(),
        cargo["package"]["version"].as_str()
    );
    let languages = manifest["language_servers"]["openplanet-lsp"]["languages"]
        .as_array()
        .unwrap();
    for (directory, name, suffix) in [
        ("angelscript", "AngelScript", "as"),
        ("openplanet_manifest", "Openplanet Manifest", "info.toml"),
    ] {
        let config: toml::Value = toml::from_str(
            &fs::read_to_string(format!("languages/{directory}/config.toml")).unwrap(),
        )
        .unwrap();
        assert_eq!(config["name"].as_str(), Some(name));
        assert!(
            languages
                .iter()
                .any(|language| language.as_str() == Some(name))
        );
        assert_eq!(
            config["path_suffixes"].as_array().unwrap(),
            &[toml::Value::String(suffix.into())]
        );
        assert!(
            manifest["grammars"]
                .get(config["grammar"].as_str().unwrap())
                .is_some()
        );
    }
    assert!(Path::new("UNLICENSE").is_file());
    assert!(fs::read_to_string("LICENSE-CC0").unwrap().len() > 6000);
}
