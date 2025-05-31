# TeaCat
This crate contains basic tools for working with TeaCat files. 

```rust
use teacat_lib::prelude::*;

let teacat_str = "
&title := :title[My Webpage];

:head[
	&title
]

:body[
	:p[Hello, World!]
]
"
.to_string();

let tokenstream = TokenStream::lex(teacat_str);
let ast = Ast::parse(tokenstream)?;
let expanded = ExpandedAst::expand(ast)?;
let html = HtmlRenderer::render(expanded);

assert_eq!(
	html,
	"<!DOCTYPE html><html><head><title>My Webpage</title></head><body><p>Hello, World!</p></body></html>".to_string()
);
```
