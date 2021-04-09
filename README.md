# Project worldlang
![GitHub](https://img.shields.io/github/license/heartleth/worldlang?style=for-the-badge)
![GitHub last commit](https://img.shields.io/github/last-commit/heartleth/worldlang?style=for-the-badge)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/heartleth/worldlang?style=for-the-badge)
![Visual Studio Marketplace Installs](https://img.shields.io/visual-studio-marketplace/i/myjmyjmyj.wlsupport?style=for-the-badge)
[![Tweet](https://img.shields.io/twitter/url/http/shields.io.svg?style=social)](https://twitter.com/intent/tweet?text=Write%20code%20once%20and%20do%20everything!&url=https://github.com/heartleth/worldlang)

<p align=center>
<img src="img/WL.png">
</p>

## What is worldlang?
_Worldlang_ is just language notation of json. How do `if` statements expressed, `while` statements, and some basic syntaxes.

The codes are [English++](https://github.com/heartleth/enpp-rust), and it will transpiled into another language, wrotten in `language.json`.

You can also use another language, if there is parser to transpile.

## English++
_English++_ is a programming language, that everyone can read, learn and write easily. Basic syntaxes are written [here](https://github.com/heartleth/enpp-rust).

The _English++_ was only able to transpile into `C++` language because the syntaxes were hard-coded in the format like, `if ({exp}) {{{block}}}`. But, by _Worldlang_, now it can transpiled into most languages.

Please contribute more engines here.

## External engines
But, _Worldlang_ can't express every languages. _Python_, uses _indents_ as a block, so _Worldlang_ and _English++_ can't write python codes.

So, `External engines` will help us. Python library [Bython]([https://github.com/mathialo/bython]) helped me. To run `Bython` can make Complete _Python_ code.

## Tutorial
be written in [template.jsonc](template.jsonc). If you can, you don't even need to read it. It's just a manual how to transpile. If there is `if true` code, the parser engine will find `(root).blocks.if` and apply it.

We need more. Some languages print string in `stdout` with `print`, some is `console.log`, etc. So `worldlib` help us. Implementation is written [here](stdlib_proto.md).

Just with _C++_, _Node.js_, _PHP_, _Python_, it's enough to write great codes.

## Supports

* Stable, Completely transpiled
    * C++
    * Node.js
* Stable, Incompletely transpiled
    * PHP
* Working in progress
    * Python
    * Kotlin
    * Ruby