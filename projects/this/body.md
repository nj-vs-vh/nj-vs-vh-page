i like inventing bicycles so for my little personal project collection page I'm writing
a custom Static Site Generator in Rust. it doesn't really do anything cool yet but I like
the idea of having a fully customizable thing.

greatly inspired by [fasterthanlime blog](https://fasterthanli.me/)

## features
- project descriptions are written in **mark**_down_ (with some custom extensions)
- code blocks with highlighting by [highlight.js](https://highlightjs.org/):
  ```python
    def greet(name: str):
        print(f"hello, {name}")
  ```
- latex-style math with [MathJax](https://www.mathjax.org/):
$\displaystyle \int_{-\infty}^{\infty} e^{x^2} dx$
- per-project media files:
![photo example](media/photo-example.jpg)
