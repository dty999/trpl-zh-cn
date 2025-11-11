## `match` 控制流结构

Rust 提供了一个强大的 `match` 控制流运算符，它允许我们将一个值与一系列模式进行比较，并根据匹配的模式执行相应代码。模式可以是字面值、变量、通配符等多种形式（所有模式类型将在[第十九章][ch19-00-patterns]详细介绍）。`match` 的强大之处在于模式的表现力和编译器确保所有可能情况都被覆盖的能力。

可以把 `match` 表达式想象成一个形状分类器：不同形状的积木通过分类器时，每个积木都会落入匹配其形状的槽中。同样，值会依次与 `match` 的每个模式比较，当遇到第一个匹配的模式时，就会执行对应的代码块。

让我们用一个几何形状的例子来说明 `match` 的用法。我们可以编写一个函数来识别不同的形状并返回其边数，如示例 6-3 所示：

```rust
enum Shape {
    Triangle,
    Square,
    Pentagon,
    Circle,
}

fn sides_count(shape: Shape) -> u8 {
    match shape {
        Shape::Triangle => 3,
        Shape::Square => 4,
        Shape::Pentagon => 5,
        Shape::Circle => 0,
    }
}
```

<span class="caption">示例 6-3：一个枚举和一个以枚举变体作为模式的 `match` 表达式</span>

让我们分析 `sides_count` 函数中的 `match` 表达式。首先，`match` 关键字后跟一个表达式，这里是 `shape` 的值。这与 `if` 的条件表达式类似，但有个重要区别：`if` 要求表达式返回布尔值，而 `match` 可以接受任何类型。这里的 `shape` 是上面定义的 `Shape` 枚举。

`match` 的每个分支有两部分：模式和相关代码。第一个分支的模式是 `Shape::Triangle`，`=>` 运算符分隔模式和要执行的代码，这里是返回值 `3`。各分支间用逗号分隔。

执行 `match` 时，会按顺序将 `shape` 与每个模式比较。当找到匹配的模式时，执行对应代码并返回结果。这就像形状分类器，每个形状会匹配对应的分支。示例中的 `match` 有四个分支，分别对应四种形状。

每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 `match` 表达式的返回值。

如果分支代码较短的话通常不使用大括号，正如示例 6-3 中的每个分支都只是返回一个值。如果想要在分支中运行多行代码，可以使用大括号，而分支后的逗号是可选的。例如，如下代码在每次使用`Coin::Penny` 调用时都会打印出 “Lucky penny!”，同时仍然返回代码块最后的值，`1`：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### 绑定值的模式

匹配分支的一个重要功能是可以绑定匹配模式的部分值，从而从枚举变体中提取数据。让我们通过一个几何形状的例子来说明这一点。

假设我们有一些带有颜色的几何形状，我们可以修改 `Shape` 枚举，让 `Circle` 变体包含颜色信息，如示例 6-4 所示：

```rust
#[derive(Debug)] // 这样可以方便地打印颜色
enum Color {
    Red,
    Green,
    Blue,
}

enum Shape {
    Triangle,
    Square,
    Pentagon,
    Circle(Color),  // Circle现在包含颜色信息
}
```

<span class="caption">示例 6-4：`Circle` 变体也存放了一个 `Color` 值的 `Shape` 枚举</span>

现在，我们可以编写一个函数，在识别形状的同时，还能获取圆形的颜色信息。在匹配表达式中，我们可以在匹配 `Shape::Circle` 的分支中绑定颜色变量：

```rust
fn describe_shape(shape: Shape) -> String {
    match shape {
        Shape::Triangle => String::from("三角形有3条边"),
        Shape::Square => String::from("正方形有4条边"),
        Shape::Pentagon => String::from("五边形有5条边"),
        Shape::Circle(color) => {
            format!("圆形有0条边，颜色是{:?}", color)
        }
    }
}
```

当调用 `describe_shape(Shape::Circle(Color::Green))` 时，`shape` 将是 `Shape::Circle(Color::Green)`。`match` 表达式会依次比较每个分支，直到匹配到 `Shape::Circle(color)`。这时，`color` 会绑定到 `Color::Green`，然后我们可以在格式化字符串中使用这个值。

这种模式匹配和值绑定的组合非常强大，可以让我们在处理枚举时同时提取和使用内部的数据。

### 匹配 `Option<T>`

`Option<T>` 是 Rust 中表示可选值的枚举类型，我们可以使用 `match` 来处理它，就像处理其他枚举一样。`match` 的工作机制完全相同，只是现在处理的是 `Option<T>` 的变体。

假设我们需要编写一个函数，它接收一个 `Option<i32>` 参数：如果有值则加一返回，如果无值则返回 `None`。使用 `match` 可以轻松实现这个功能：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">示例 6-5：在 `Option<i32>` 上使用 `match` 表达式</span>

让我们逐步分析 `plus_one` 函数的工作原理：

1. 当调用 `plus_one(five)` 时，`x` 的值是 `Some(5)`
2. `match` 会依次检查每个分支：
   - 第一个分支 `None => None` 不匹配 `Some(5)`
   - 第二个分支 `Some(i) => Some(i + 1)` 匹配成功
3. 匹配成功后，`i` 会绑定到 `Some` 中的值 `5`
4. 执行分支代码 `i + 1` 得到 `6`，然后包装成 `Some(6)` 返回

当传入 `None` 时：
1. `x` 的值是 `None`
2. 第一个分支 `None => None` 直接匹配成功
3. 返回 `None`，其他分支不再检查

这种处理方式确保了无论 `Option<T>` 是 `Some` 还是 `None`，都能得到正确的处理。

将 `match` 与枚举相结合在很多场景中都是有用的。你会在 Rust 代码中看到很多这样的模式：`match` 一个枚举，绑定其中的值到一个变量，接着根据其值执行代码。这在一开始有点复杂，不过一旦习惯了，你会希望所有语言都拥有它！这一直是用户的最爱。

### 匹配是穷尽的

`match` 还有另一方面需要讨论：这些分支必须覆盖了所有的可能性。考虑一下 `plus_one` 函数的这个版本，它有一个 bug 并不能编译：

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

我们没有处理 `None` 的情况，所以这些代码会造成一个 bug。幸运的是，这是一个 Rust 知道如何处理的 bug。如果尝试编译这段代码，会得到这个错误：

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust 知道我们没有覆盖所有可能的情况甚至知道哪些模式被忘记了！Rust 中的匹配是 **穷尽的**（*exhaustive*）：必须穷举到最后的可能性来使代码有效。特别的在这个 `Option<T>` 的例子中，Rust 防止我们忘记明确的处理 `None` 的情况，这让我们免于假设拥有一个实际上为空的值，从而使之前提到的价值亿万的错误不可能发生。

### 通配模式和 `_` 占位符

使用枚举，我们也可以针对少数几个特定值执行特殊操作，而对其他所有值采取默认操作。想象我们正在玩一个游戏，如果你掷出骰子的值为 3，角色不会移动，而是会得到一顶新奇的帽子。如果你掷出了 7，你的角色将失去一顶新奇的帽子。对于其他的数值，你的角色会在棋盘上移动相应的格子。这是一个实现了上述逻辑的 `match`，骰子的结果是硬编码而不是一个随机值，其他的逻辑部分使用了没有函数体的函数来表示，实现它们超出了本例的范围：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

对于前两个分支，匹配模式是字面值 `3` 和 `7`，最后一个分支则涵盖了所有其他可能的值，模式是我们命名为 `other` 的一个变量。`other` 分支的代码通过将其传递给 `move_player` 函数来使用这个变量。

即使我们没有列出 `u8` 所有可能的值，这段代码依然能够编译，因为最后一个模式将匹配所有未被特殊列出的值。这种通配模式满足了 `match` 必须被穷尽的要求。请注意，我们必须将通配分支放在最后，因为模式是按顺序匹配的。如果我们在通配分支后添加其他分支，Rust 将会警告我们，因为此后的分支永远不会被匹配到。

Rust 还提供了一个模式，当我们不想使用通配模式获取的值时，请使用 `_` ，这是一个特殊的模式，可以匹配任意值而不绑定到该值。这告诉 Rust 我们不会使用这个值，所以 Rust 也不会警告我们存在未使用的变量。

让我们改变游戏规则：现在，当你掷出的值不是 3 或 7 的时候，你必须再次掷出。这种情况下我们不需要使用这个值，所以我们改动代码使用 `_` 来替代变量 `other` ：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

这个例子也满足穷尽性要求，因为我们在最后一个分支中显式地忽略了其它值。我们没有忘记处理任何东西。

最后，让我们再次改变游戏规则，如果你掷出 3 或 7 以外的值，你的回合将无事发生。我们可以使用单元值（在[“元组类型”][tuples]<!-- ignore -->一节中提到的空元组）作为 `_` 分支的代码：

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

在这里，我们明确告诉 Rust 我们不会使用与前面模式不匹配的值，并且这种情况下我们不想运行任何代码。

我们将在[第十九章][ch19-00-patterns]<!-- ignore -->中介绍更多关于模式和匹配的内容。现在，让我们继续讨论 `if let` 语法，这在 `match` 表达式显得有些冗长时非常有用。

[tuples]: ch03-02-data-types.html#元组类型
[ch19-00-patterns]: ch19-00-patterns.html
