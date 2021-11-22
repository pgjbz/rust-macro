Install:

- https://github.com/togglebyte/nvim
- https://www.linode.com/docs/guides/how-to-install-neovim-and-plugins-with-vim-plug/

And copy and pas init.vim to ~/.config/nvim

Open nvim and run 'PlugInstall'

Use command 'CocInstall coc-rust-analyzer'

To install Rust analyzer

# Macro rules

Captures are written as a dollar ($) followed by an identifier, a colon (:), and finally the kind of capture, which must be one of the following:

- item: an item, like a function, struct, module, etc.
- block: a block (i.e. a block of statements and/or an expression, surrounded by braces)
- stmt: a statement
- pat: a pattern
- expr: an expression
- ty: a type
- ident: an identifier
- path: a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
- meta: a meta item; the things that go inside #[...] and #![...] attributes
- tt: a single token tree


Minutiae

Complete list of capturing matchs 'Rust 1.3'

- item: anything
- block: anything
- stmt: => , ;
- pat: => , = if in
- expr: => , ;
- ty: , => : = > ; as
- ident: anything
- path: , => : = > ; as
- meta: anything
- tt: anything
