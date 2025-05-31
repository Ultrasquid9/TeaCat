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

## Features
- [x] Elements
- [x] Attributes
- [x] Variables
- [x] Comments
- [x] Macros
- [x] Arrays
- [ ] Inline CSS/JS
- [ ] Syntax Highlighting 

## FAQ
#### Why create another language?
I wanted to create a website, but was frustrated by the how difficult to read and write HTML was. While similar languages exist, I have been wanting to try creating my own language for a long time now, and thought that this was a good excuse to try. 

#### Do I need a webserver to use TeaCat?
Nope! TeaCat is used entirely offline.

#### Can I use a webserver to use TeaCat?
Currently, there is no official way to dynamically convert TeaCat into HTML. However, if this is requested enough, this may change in the future!

#### Where did the name "TeaCat" come from?
The language was initially named "WebCat", as I intended to use it to create a website, and I like cats. However, the name was already taken on crates.io, and there are many projects on Github already named WebCat. I cosidered "Template Cat", but shortened that down to "TCat", and that naturally turned into "TeaCat" over time. 

#### Can I use TeaCat in my projects?
Currently, TeaCat is still a work in progress, and there are no guarantees of stability as of now. However, if you want to use it, feel free to do so! 
