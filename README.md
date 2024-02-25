# Filecoin VM 参考实现（v4； dev）

[![持续集成](https://github.com/filecoin-project/ref-fvm/actions/workflows/ci.yml/badge.svg)](https://github.com/filecoin-project/ref-fvm/actions/workflows/ci.yml)

本仓库包含Filecoin VM的参考实现（[规范](https://github.com/filecoin-project/fvm-project)）。它使用Rust编写，并旨在通过FFI集成到非Rust客户端（例如，Lotus、Fuhon），或直接集成到Rust客户端（例如，Forest）。仓库中提供了Go的FFI绑定，并鼓励开发者为其他语言贡献绑定。

有关详细信息，请参见[项目网站](https://fvm.filecoin.io/)。

## 构建要求

* 安装[rustup](https://rustup.rs/)。

## 构建指南

```sh
$ git clone https://github.com/foxxcn/RefFVM.git
$ cd RefFVM
$ make
```

## 代码结构

以下是每个目录中你会发现的内容：

- `/fvm`
  - Filecoin虚拟机的核心。关键概念包括：
    - `Machine`：机器的一个实例，固定在特定的状态根和纪元，准备接收要应用的消息。
    - `Executor`：在`Machine`上执行消息的对象。
    - `CallManager`：跟踪和管理给定消息的调用栈。
    - 调用容器（概念层面，代码中不明确出现）：给定调用栈中的参与者运行的WASM实例+沙盒。
    - `Kernel`：附加到调用容器的环境，用于外部交互。
  - 系统中有两个API边界：
    1. 参与者代码与Kernel之间的边界，通过调用`Syscalls`来跨越。
    2. FVM与宿主节点之间的边界，由`Externs`表示。
  - FVM的某些部分基于[Forest](https://github.com/ChainSafe/forest)实现。
- `/sdk`
  - 用于编写Filecoin原生参与者的参考SDK实现，通过参与者FVM运行时垫片被标准内置参与者使用。
  - 使用Rust编写的用户定义的FVM参与者也可以使用此SDK，尽管目前它还相当粗糙。在接下来的几周内，我们预计会改善它，以提升开发者体验。
  - 社区中将出现其他SDK。我们也预期社区团队将开发其他WASM可编译语言的SDK，如Swift、Kotlin（使用Kotlin Native）甚至Go（通过TinyGo编译器）。
- `/shared`
  - 一个在FVM和SDK之间共享的核心类型和原始类型的箱（crate）。
- `/ipld`
  - IPLD库。其中一些基于并改编自[Forest](https://github.com/ChainSafe/forest)实现。
- `/testing/conformance`
  - 包含测试向量运行器以及在其上的基准测试实用工具。
  - 符合性测试运行器将位于https://github.com/filecoin-project/fvm-test-vectors的测试向量语料库输入ref-fvm，以验证规范符合性。
  - 基准测试工具使用`criterion` Rust库来衡量ref-fvm在各个方面的性能和开销。
  - 请参阅[说明](./testing/conformance/README.md#instructions)了解如何运行测试和基准测试。
  - 免责声明
    - 基准测试目前运行、设置和拆除非常慢。这是因为使用了默认的WASM缓存，我们将很快修复这个问题。

## 许可证

双重许可： [MIT](./LICENSE-MIT), [Apache Software License v2](./LICENSE-APACHE), 通过
[Permissive License Stack](https://protocol.ai/blog/announcing-the-permissive-license-stack/).

---

参与者和虚拟机分叉自[ChainSafe/forest](https://github.com/ChainSafe/forest)
提交：[`73e8f95a108902c6bef44ee359a8478663844e5b`](https://github.com/ChainSafe/forest/commit/73e8f95a108902c6bef44ee359a8478663844e5b)