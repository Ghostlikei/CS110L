# Lecture 2: Memory Safety

## Dangling Pointers

两种常见的危险类型

```c
// Case 1
vec = NULL;
return &vec
  
// Case 2
// Double free
free(vec->data);
vec_free(vec);

// Case 3
// Iterator Invalidation
vec_push(vec, 0xff);
int *n = &vec->data[0];
vec_push(vec, 0x8f);
printf("%d", *n);//bad
```

- Case1: 直接在函数内返回地址会导致失去对于地址的控制权，会被其他程序覆盖

- Case2: 在free一次之后，原先的地址可能被其他的程序占用，再次free会引发堆内存错误

- Case3: vector会resize，原先的地址空间被释放，这个时候n指针会指向一个被释放的空间，出错（使用迭代器但是更改数据结构本身）

## It is incredibly hard to reason about programs

- Do I have a secure voting machine?
- 如果rust不知道这些问题怎么解决，他是怎么找到这些问题的？

## Rust

- 通常来说，我们是在安全的rust下coding的，这是通过加入一些限制和规则来实现的，有的时候需要编写*不安全*的rust程序

- 例如对于上面三个例子，rust会在编译阶段直接不允许这样做

### Ownership

- 简单来说，每个值有且只有一个owner，当owner消失，value会被**丢掉**（如果是文件，文件会关闭，是数据结构就deconstruct，是内存就free）
- rust有堆指针来进行引用计数 

例子：移动构造权限问题

```rust
fn main(){
	let s: String = String::new();
  let u = s;// Transferring Ownership
  println!("{}",s);//crush!!
  println!("{}",s);//ok
}
```

因为没有实现拷贝构造，u默认是移动构造，在移动构造之后**权限被转移**，不能再控制s了（owner不再是main函数）

```rust
fn om_nom_nom(s: String) {
  println!("I have comsumed {}", s);
}

fn main(){
  let s: String= String::new();
  om_nom_nom(s);// The ownership of s is transferred to om_nom_nom
  println!("{}", s);// Crush!!!!
}
```

想要交还控制权的话使用return

- 对于整数，无符号整数来说，`=`会更改其本来的功能，变成拷贝赋值（把上面的string改成i32就能编译通过并且运行）

#### Why rust implemented ownership?

- 律师问题

你有一群律师和一个合同，需要设定哪些基本的规则来避免混乱

1.最多只能让一个律师签字

2.a律师在读的时候，其他所有人都不能写

3.b律师在写的时候，其他所有人都不能写

#### Borrowing situation

- 可以同时有多个const pointer
- 但是一旦有了一个non-const pointer，必须扔掉其他所有const pointer（否则就会变成野指针）
- 最多只能有一个non-const pointer

#### Shared Borrow

- 可以同时有多个不可更改的（immutable）引用（reference），但是这个时候不能有mutable reference
- 有mutable reference时，不能有其他引用



### Lifetime

- rust会找到一个值的起始和终止，终止后会掉用drop方法，对于这个值的引用在生命周期结束之后会报废
- rust使用的是静态分析的技术

  





