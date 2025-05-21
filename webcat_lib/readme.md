# WebCat
This crate contains basic tools for working with WebCat files. 

```rust
use webcat_lib::prelude::*;
use std::collections::HashMap;

let webcat_str = "
&title := :title[My Webpage];

:head[
	&title
]

:body[
	:p[
		Hello, World!
	]
]
"
.to_string();

let tokenstream = TokenStream::lex(webcat_str);
let ast = Ast::parse(tokenstream);
let html = Html::expand(ast, &HashMap::new());

assert_eq!(
	html.render(),
	"<head><title>My Webpage</title></head><body><p>Hello, World!</p></body>".to_string()
);
```
