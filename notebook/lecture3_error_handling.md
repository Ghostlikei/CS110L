# Lecture3: Ownership and Error handling

## 'Ownership' in C

- 在C语言当中，没有真正的ownership概念，有借用(const \*)和引用(\* const)的概念

1.C语言当中如果函数用的是借用，那么这个函数不需要负责原来的指针的释放

2.如果用的是引用，需要释放指针的话可能会用free函数，也可以交还控制权，不释放

3.有一种情况就是原先值的指针不能用简单的`free`函数来释放，需要用内置的`deconstruct`方法，这个时候需要把释放函数指针一起传到函数内

4.释放指针的函数涉及到`struct`，这个时候不仅要释放该指针的空间，还要释放该指针指向的空间，没有ownership的概念会变得非常麻烦（比如说链表或者二叉树）



 ## Compile time & Run time

- Compile time

Rust 会在传递ownership的时候加入合理的free函数

如果是引用，仅仅是传递指针

拷贝赋值的时候，仅仅是浅拷贝 



## Ownership Examples

- Does it compile?

```rust
// case 
fn main(){
  let s = String::from("Hello");
  let s1 = s;
  let s2 = s;
  println!("{} {} {}", s1, s2, s);
}
// 编译失败，所有权从s移动到了s1，s2不能再使用了

// case 2
fn main(){
  let s = String::from("Hello");
  let s1 = &s;
  let s2 = &s;
  println!("{} {} {}", s1, s2, s);
}
// 编译通过，因为s1和s2只是看s，并不会更改，并不违反Ownership原则

// case 3
fn main(){
  let s = String::from("Hello");
  let s1 = &mut s;
  let s2 = &s;
  println!("{} {} {}", s1, s2, s);
}
// 编译失败，因为s1在改的时候，s2在看，违反了规则

// case 4
fn main(){
  let s = String::from("Hello");
  let s1 = &mut s;
  println!("{} {}", s1, s);
}
// 编译失败，immutable变量不能传mutable指针

// case 5
fn main(){
  let mut s = String::from("Hello");
  let s1 = &mut s;
  println!("{} {}", s1, s);
}
// 编译失败，s和s1同时有更改权，违反规则

// case 6
fn main(){
  let mut s = String::from("Hello");
  let s1 = &mut s;
  println!("{} ", s1);
  println!("{} ", s);
}
// 编译通过，rust编译器自动把s1的权限交还给了main函数

// case 7
fn main(){
  let mut s = String::from("Hello");
  let s1 = &mut s;
  println!("{} ", s);
  println!("{} ", s1);//这里因为s1是可变的，println函数会对s1使用mutable borrow（对结果不影响）
}
// 编译失败，在print(s)的时候发生了immutable reference，但这个时候还有一个mutable reference s1，不符合规则

```

 ## Error handling

### NULL pointer

```c
size_t len = package.length();
void* buf = malloc(len);
memcpy(buf, package.data, len);
```

如果len超过了内存的大小，就会返回一个**空指针**，memcpy就会对空指针解引用(null pointer dereference)，这是**非常非常不好的**

- `rust`里的解决办法

`Option<T>`:可以使函数返回`None`,也可以返回T类型的一个值:`Some(T);`

```rust
fn func() -> Option<T> {
	do_something();
	if condition {
		Some(T);
	}
	else{
		None;
	}
}
func().is_none(); // return 1 if not none and return 0 if none
func().unwrap_or(Anything);// if some return some, if none return Anything;

match func(){
  Some(message) =>{
    dosth();
  },
  None => {
    dosth();
  },
}
```







