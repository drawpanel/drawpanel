# Drawpanel

[中文文档](./readme_zh.md)

A drawing engine written in rust.

## Develop

### Packages

| Package                      | Note                                                                                                                                                   | Type   |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | ------ |
| packages/drawpanel           | This is a 2D drawing tool finally implemented by other components.                                                                                     | app    |
| packages/drawpanel-core      | This is where most functions are implemented. Such as components, scaling, moving, etc.                                                                | core   |
| packages/drawpanel-bind-fltk | This is an auxiliary layer bound with fltk. It is used to pass the events sent by fltk to drawpanel core, and call the drawing method of fltk to draw. | binder |
| packages/drawpanel-bind-wasm | This is the auxiliary layer bound with wasm, and is finally applied to the web.                                                                        | binder |
| packages/drawpanel-bind-\*   | Binding with other gui libraries.                                                                                                                      | binder |

### Start

```shell
git clone https://github.com/drawpanel/drawpanel
cd drawpanel
cargo run --package=drawpanel # cargo watch -x 'run' -w ../drawpanel
```

## Example

> Run Example:

```shell
cargo run --example=app -p drawpanel-bind-fltk
cargo run --example=app -p drawpanel-bind-egui
```

### Egui

[Demo video](packages/drawpanel-bind-egui/readme.md)

### Contribute

It can be directly conducted through PR.

## License

MIT
