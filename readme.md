# TeaCat
HTML is a pain to write. It's wordy, messy, and generally not something you want to be doing yourself. TeaCat is designed to solve this.

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
TeaCat allows you to instead write it like this:
```
&hello_world := Hello World!;

:head[
	:title[&hello_world]
]

:body[
	:h1[&hello_world]
	:p[Welcome to my website.]
]
```
The following TeaCat code can then be instantly transpiled into HTML entirely offline, no JS or webserver required. 

Features:
- [x] Elements
- [x] Attributes
- [x] Variables
- [x] Comments
- [x] Macros
- [ ] Arrays
- [ ] Inline CSS/JS
- [ ] Syntax Highlighting 
