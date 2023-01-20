# Drawpanel

一个用 rust 编写的画图引擎。

## 开发

### 结构

| 位置                         | 说明                                                                                                    | 类型 |
| ---------------------------- | ------------------------------------------------------------------------------------------------------- | ---- |
| packages/drawpanel           | 这是一个用其他组件最终实现的 2D 绘图工具。                                                              | 产品 |
| packages/drawpanel-core      | 这是绝大多数的功能实现的地方。比如组件、缩放、移动等。                                                  | 核心 |
| packages/drawpanel-bind-fltk | 这是与 fltk 绑定的辅助层，用来传递 fltk 发送的事件到 drawpanel-core，并且调用 fltk 的绘图方法进行绘制。 | 绑定 |
| packages/drawpanel-bind-wasm | 这是与 wasm 绑定的辅助层，最终应用在 web。                                                              | 绑定 |
| packages/drawpanel-bind-\*   | 与其他 gui 库的绑定。                                                                                   | 绑定 |

### 开始

```shell
git clone https://github.com/drawpanel/drawpanel
cd drawpanel
cargo run --package=drawpanel # cargo watch -x 'run' -w ../drawpanel
```

### 贡献

可以直接通过 PR 进行，目前规范正在完善中。

## 协议

MIT
