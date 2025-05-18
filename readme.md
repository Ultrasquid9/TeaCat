# WebCat
HTML is a pain to write. It's wordy, messy, and generally not something you want to be doing yourself. WebCat is designed to solve this.

For example, take the following HTML:
```html
<html>
	<head>
		<title>Hello World!</title>
	</head>
	
	<body>
		<h1>Hello World!</h1>
		<p>Welcome to my website.</p>
	</body>
</html>

```
WebCat allows you to instead write it like this:
```
$hello_world <- Hello World!;

:head[
	:title[&hello_world]
]

:body[
	:h1[&hello_world]
	:p[Welcome to my website.]
]
```
The following WebCat code can then be instantly transpiled into HTML entirely offline, no JS or webserver required. 

Features:
- [x] Elements
- [ ] Attributes
- [ ] Variables
- [x] Comments
- [ ] Macros
- [ ] Inline CSS/JS
- [ ] Syntax Highlighting 
