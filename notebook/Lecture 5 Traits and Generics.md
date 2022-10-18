# Lecture 5: Traits and Generics

## Traits

### 实现方法

1.手动重写traits：`impl TRAIT for STRUCT {...}`

2.简单结构体下告诉编译器，可以用这些方法`#[derive(Debug, Clone,....)]`

3.自己定义一个trait：`pub trait computeNorm { fn compute_norm(&self) {...} }`

### Traits的种类

- Display
- Clone/Copy

​	区别在于`a.clone()`会提供一个全新的a，但是copy会重写赋值运算符`=`，变成类似于拷贝构造

- Iterator
- Equal/Partital Equal

Equal只适用于**类型内所有的东西和自己相等（例如int）**

Partital是一个弱化的条件，比如f64当中有一个元素NaN != NaN

Override

- Drop
```rust
impl Drop for LinkedList {
	fn drop(&mut self) {
		let mut current = self.head.take();
		while let Some(mut node) = current {
      current = node.next.take(); 
    }
	}
}
```
- Deref
- Default implementation 

- ToString

Operator Overload

- Add ...

```rust
impl Add for Point {
	type Output = Self; // "associated type"
	fn add(self, other: Self) -> Self {
		...
	}
}
```



## Generics

```rust
pub struct Pair<T> {
  first: T,
  second: T
}

pub enum MyOption<T> {
  Sumthin(T), Nuthin
}

impl fnt::Display for MyOption<u32> {
  ...
}// How to avoid copy/paste for char?

impl<T> Pair<T> {
  ...
}

impl<T: fmt:Display> fmt::Display for MyOption<T> {
  ...
}

//where syntax
impl<T> fmt::Display for Pair<T> where T: fmt::Display {
  
}
```



