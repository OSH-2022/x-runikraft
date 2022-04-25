# Rust语言的优越性

这一部分主要阐释我们为什么选择Rust语言作为开发语言。Rust 是由 Mozilla 研究室主导开发的一门现代系统编程语言，自 2015 年 5 月发布 1.0 之后，一直以每 6 周一个小版本的开发进度稳定向前推进。语言设计上跟 C++ 一样强调零开销抽象和 RAII。拥有极小的运行时和高效的 C 绑定，使其运行效率与 C/C++ 一个级别，非常适合对性能要求较高的系统编程领域。利用强大的类型系统和独特的生命周期管理实现了编译期内存管理，保证内存安全和线程安全的同时使编译后的程序运行速度极快，Rust 还提供函数式编程语言的模式匹配和类型推导，让程序写起来更简洁优雅。宏和基于 trait 的泛型机制让 Rust 的拥有非常强大的抽象能力，在实际工程中尤其是库的编写过程中可以少写很多 boilerplate 代码。[^2]总地来说，Rust是一门赋予每个人 构建可靠且高效软件能力的语言。[^1]它具有以下三个方面的优越性：

## 高性能（Performance）

Rust 速度惊人且内存利用率极高。由于没有运行时和垃圾回收，它能够胜任对性能要求特别高的服务，可以在嵌入式设备上运行，还能轻松和其他语言集成。[^1]

### 可执行文件

Rust是编译语言，这意味着程序直接转换为可执行的机器代码，因此可以将程序作为单个二进制文件进行部署；与 Python 和 Ruby 等解释型语言不同，无需随程序一起分发解释器，大量库和依赖项，这是一大优势。与解释型语言相比，Rust 程序非常快。[^4]

### 对动态类型语言与静态类型的平衡

动态类型语言在调试、运行时具有不确定性，而静态类型的语言减少了程序理解的开销和动态运行的不确定性，但并不是所有静态类型系统都是高效的。Rust使用可选类型对这种可能性进行编码，并且编译器要求你处理`None`这种情况。这样可以防止发生可怕的运行时错误（或等效语言），而可以将其提升为你在用户看到它之前解决的编译时错误。Rust的静态类型尽最大努力避免程序员的麻烦，同时鼓励长期的可维护性。一些静态类型的语言给程序员带来了沉重的负担，要求他们多次重复变量的类型，这阻碍了可读性和重构。其他静态类型的语言允许在全局进行数据类型推断。虽然在最初的开发过程中很方便，但是这会降低编译器在类型不再匹配时提供有用的错误信息的能力。Rust可以从这两种样式中学习，并要求顶层项（如函数参数和常量）具有显式类型，同时允许在函数体内部进行类型推断。[^3]

### 解决垃圾回收问题

Rust可以选择将数据存储在堆栈上还是堆上，并在编译时确定何时不再需要内存并可以对其进行清理。这样可以有效利用内存，并实现更高性能的内存访问。Tilde是Rust在其Skylight产品中的早期生产用户，他发现通过使用Rust重写某些Java HTTP服务，他们能够将内存使用量从5Gb减少到50Mb。无需连续运行垃圾收集器，Rust项目非常适合被其他编程语言通过外部功能接口用作库。这使现有项目可以用快速的Rust代码替换对性能至关重要的代码，而不会产生其他系统编程语言固有的内存安全风险。某些项目甚至已使用这些技术在Rust中进行了增量重写。通过直接访问硬件和内存，Rust是嵌入式和裸机开发的理想语言你您可以编写底层代码，例如操作系统内核或微控制器应用程序。在这些特别具有挑战性的环境中，Rust的核心类型和功能以及可重用的库代码表现将会非常出色。[^3]



## 可靠性（Reliability）

Rust 丰富的类型系统和所有权模型保证了内存安全和线程安全，让您在编译期就能够消除各种各样的错误。[^1]

### 处理系统级编程

与其他系统级编程语言（例如C或C ++）相比，Rust可以提供的最大好处是借阅检查器。这是编译器的一部分，负责确保引用不会超出引用的数据寿命，并有助于消除由于内存不安全而导致的所有类型的错误。与许多现有的系统编程语言不同，Rust不需要你将所有时间都花在细节上。Rust力求拥有尽可能多的*零成本抽象*，这种抽象与等效的手写代码具有同等的性能。当安全的Rust无法表达某些概念时，ref="[http://cliffle.com/p/dangerust/](https://link.zhihu.com/?target=http%3A//cliffle.com/p/dangerust/)">可以使用不安全的 Rust。这样可以释放一些额外的功能，但作为交换，程序员现在有责任确保代码真正安全。然后，可以将这种不安全的代码包装在更高级别的抽象中，以确保抽象的所有使用都是安全的。使用不安全的代码应该是一个经过深思熟虑的决定，因为正确使用它需要与负责避免未定义行为的任何其他语言一样多的思考和关心。最小化不安全代码是最小化由于内存不安全而导致段错误和漏洞的可能性的最佳方法。系统性编程语言有一种天生的使命，即它们必须永远有效地存在。尽管某些现代软件并不需要那么长的使用寿命，但许多企业希望其辛苦编写代码库在可预见的将来能够一直使用。[^3]

### Concurrent programming made easier

Rust makes it easier to write concurrent programs by preventing data races at compile time. A data race occurs when at least two different instructions from different threads are trying to access the same memory location simultaneously, while at least one of them is trying to write something and there is no synchronization that could set any particular order among the various accesses. Access to the memory without synchronization is undefined. In Rust, data races are detected. If a given object access does not support many threads ( i. e. is not marked with an appropriate trait), it needs to be synchronized by a mutex that will lock access to this particular object for other threads. To ensure that operations performed on an object will not break it, only one thread has access to it. From the perspective of other threads, operations on this object are atomic, which means that an observed state of the object is always correct and you cannot observe any intermediate state resulting from an operation performed on this object by another thread. Rust language can check if we are performing any incorrect operations on such objects and inform us at compile time.[^6]

## 生产力（Productivity）

Rust 拥有出色的文档、友好的编译器和清晰的错误提示信息， 还集成了一流的工具——包管理器和构建工具， 智能地自动补全和类型检验的多编辑器支持， 以及自动格式化代码等等。[^1]

### Cargo包管理器

Rust 由于有 Cargo 这样一个非常出色的包管理工具，周边的第三方库发展非常迅速，各个领域都有比较成熟的库，比如 HTTP 库有 Hyper，异步 IO 库有 Tokio, mio 等，基本上构建后端应用必须的库 Rust 都已经比较齐备。 总体来说，现阶段 Rust 定位的方向还是高性能服务器端程序开发，另外类型系统和语法层面上的创新也使得其可以作为开发 DSL 的利器。[^2]

Cargo is recognized as one of the exceptional strengths of the Rust ecosystem. Without Cargo, we would have had to search for libraries, download these libraries from unknown sources from GitHub, build as static library crates, link them against the program. How painful this is all to do. But we have Cargo that does all this for us while working with Rust.[^7]

## Rust语言的兼容性

The Rust language is fast evolving, and because of this certain compatibility issues can arise, despite efforts to ensure forwards-compatibility wherever possible.

Rust, like many programming languages, has the concept of "keywords". These identifiers mean something to the language, and so you cannot use them in places like variable names, function names, and other places. Raw identifiers let you use keywords where they would not normally be allowed. This is particularly useful when Rust introduces new keywords, and a library using an older edition of Rust has a variable or function with the same name as a keyword introduced in a newer edition.

For example, consider a crate `foo` compiled with the 2015 edition of Rust that exports a function named `try`. This keyword is reserved for a new feature in the 2018 edition, so without raw identifiers, we would have no way to name the function.[^8]

## 与其他语言对比

## 相比其他语言Rust的优势

### Go

Rust语言表达能力更强，性能更高，同时线程安全方面Rust也更强，不容易写出错误的代码，包管理Rust也更好，Go虽然在1.10版本后提供了包管理，但是目前还比不上Rust的。

### C++

与C++相比，Rust的性能相差无几，但是在安全性方面会更优，特别是使用第三方库时，Rust的严格要求会让第三方库的质量明显高很多。

语言本身的学习，Rust的前中期学习曲线会更陡峭，但是对于未来使用场景和生态的学习,C++会更难、更复杂。

### Java

除了极少部分纯粹的数字计算性能，Rust的性能是全面领先于Java的，同时Rust占用内存小的多，因此实现同等规模的服务，Rust所需的硬件成本会显著降低。

### Python

性能自然是Rust完胜，同时Rust对运行环境要求较低，这两点差不多就足够抉择了，因为python和rust的彼此适用面其实不太冲突。[^5]



[^1]:[Rust Programming Language](https://www.rust-lang.org/)
[^2]:[我们为什么要选择小众语言 Rust 来开发软件？](https://www.techug.com/post/why-we-choose-rust-to-dev.html)
[^3]:[What is Rust and why is it so popular? - Stack Overflow Blog](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/)
[^4]:[也许是最客观、全面的比较 Rust 与 Go：都想把 Rust 也学一下](https://www.cnblogs.com/Chary/p/14097609.html)
[^5]:[为什么要使用 Rust 语言？Rust 语言的优势在哪里？](https://www.zhihu.com/question/393796866)
[^6]:[Rust programming language - what is rust used for and why is so popular?](https://codilime.com/blog/why-is-rust-programming-language-so-popular/)
[^7]:[Rust by the Numbers: The Rust Programming Language in 2021 – The New Stack](https://thenewstack.io/rust-by-the-numbers-the-rust-programming-language-in-2021/)
[^8]:[Compatibility - Rust By Example (rust-lang.org)](https://doc.rust-lang.org/rust-by-example/compatibility.html)

