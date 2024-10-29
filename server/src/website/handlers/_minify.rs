// Files are bigger than with SWC, to retest later

// Source: https://github.com/oxc-project/oxc/blob/main/crates/oxc_minifier/examples/minifier.rs

use std::{fs, path::Path};

use oxc::{
    allocator::Allocator,
    codegen::{CodeGenerator, CodegenOptions},
    minifier::{MinifierOptions, MinifierReturn},
    parser::{Parser, ParserReturn},
    span::SourceType,
};

//
pub fn minify_js(path: &Path) -> String {
    let allocator = Allocator::default();

    let source_type = SourceType::from_path(path).unwrap();

    let source_text = fs::read_to_string(path).unwrap();

    let ParserReturn { mut program, .. } =
        Parser::new(&allocator, &source_text, source_type).parse();

    let minifier = oxc::minifier::Minifier::new(MinifierOptions::default());

    let MinifierReturn { mangler } = minifier.build(&allocator, &mut program);

    CodeGenerator::new()
        .with_options(CodegenOptions {
            single_quote: false,
            minify: true,
            comments: false,
            annotation_comments: false,
            source_map_path: None,
        })
        .with_mangler(mangler)
        .build(&program)
        .code
}
