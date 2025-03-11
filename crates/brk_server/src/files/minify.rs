// Source: https://github.com/oxc-project/oxc/blob/main/crates/oxc_minifier/examples/minifier.rs

use std::{fs, path::Path};

use oxc::{
    allocator::Allocator,
    codegen::{CodeGenerator, CodegenOptions, LegalComment},
    minifier::{CompressOptions, MangleOptions, Minifier, MinifierOptions},
    parser::Parser,
    span::SourceType,
};

pub fn minify_js(path: &Path) -> String {
    let source_text = fs::read_to_string(path).unwrap();
    let source_type = SourceType::from_path(path).unwrap();

    let allocator = Allocator::default();

    let parser_return = Parser::new(&allocator, &source_text, source_type).parse();

    let mut program = parser_return.program;

    let minifier_return = Minifier::new(MinifierOptions {
        mangle: Some(MangleOptions::default()),
        compress: Some(CompressOptions::default()),
    })
    .build(&allocator, &mut program);

    CodeGenerator::new()
        .with_options(CodegenOptions {
            minify: true,
            single_quote: false,
            comments: false,
            annotation_comments: false,
            source_map_path: None,
            legal_comments: LegalComment::None,
        })
        .with_scoping(minifier_return.scoping)
        .build(&program)
        .code
}
