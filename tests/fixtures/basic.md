# QuickMD Test Document

## Headings

This document tests all major GFM features.

### Level 3 Heading

#### Level 4 Heading

## Text Formatting

This is **bold text** and this is *italic text*. ~~Strikethrough~~ is also supported.

You can have `inline code` and **_combined_** formatting.

## Links

[GitHub](https://github.com) is where the world builds software.

Auto-linked URL: https://www.example.com

## Blockquotes

> This is a blockquote.
>
> It can span multiple lines.
>
> > And it can be nested.

## Lists

### Unordered

- Item 1
- Item 2
  - Nested item 2.1
  - Nested item 2.2
- Item 3

### Ordered

1. First item
2. Second item
   1. Nested ordered 2.1
   2. Nested ordered 2.2
3. Third item

### Task List

- [x] Completed task
- [x] Another completed task
- [ ] Pending task
- [ ] Another pending task

## Code

Inline: `const x = 42;`

### Code Block (JavaScript)

```javascript
function fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

console.log(fibonacci(10)); // 55
```

### Code Block (Python)

```python
def greet(name: str) -> str:
    """Say hello to someone."""
    return f"Hello, {name}!"

print(greet("World"))
```

### Code Block (Rust)

```rust
fn main() {
    println!("Hello, QuickMD!");
}
```

## Table

| Name   | Type    | Description          |
|--------|---------|----------------------|
| id     | integer | Primary key          |
| name   | string  | The display name     |
| email  | string  | Contact email        |
| active | boolean | Whether user is active |

### Aligned Table

| Left     | Center |    Right |
|:---------|:------:|---------:|
| Content  | Content|  Content |
| Longer   | Text   |    42.00 |

## Horizontal Rule

---

## Images

![Placeholder](https://placehold.co/600x200/0969da/white?text=QuickMD+Test)

## Emoji (via markdown-it-emoji)

:rocket: :tada: :sparkles: :heart:

## Footnotes

Here is a footnote reference[^1].

[^1]: Here is the footnote content.

## HTML

<details>
<summary>Click to expand</summary>

This content is hidden inside a `<details>` element.

</details>

---

*End of test document.*
