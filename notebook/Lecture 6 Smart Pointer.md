# Lecture 6 Smart Pointer

## More about Generics

- "Zero cost"

rust在对于泛型编程的实例化的时候会自动生成**特性的方法**，这样在runtime的时候不会有多余的时间损耗

这一条语言的特性对于impl Traits也是适用的

## 	`Box<T>`

- unique pointer：Only one thing refering to it

- limitations

比如说**图**，在runtime的时候需要通过多个指针来更改其中的内容，这个时候Box就不太好用了

## `Rc<T>`

- Shared pointer:only one mutable reference or multiple immutable reference at the same time

- 如果发生**循环引用**的时候会发生内存泄漏，需要用其他指针来解决

rust只有**引用计数器**，发生循环引用的时候并不会让计数器置零，但是如果要让编译器发现这个问题的话，需要观察**所有的内存区域**，这就涉及到垃圾回收机制，但是rust处于性能考虑是没有这样高级的策略的

## `RefCell<T>`

- Shared pointer but "lie" to the compiler by providing interior mutability
- `new()`不会在堆上开辟新的空间
- 会在runtime强制符合rust引用规则，但是会有额外的开销

“骗过”编译器，在内存上额外开辟超过编译阶段的空间，但是会有内存检查，如果发现发现unsafe就会报错

- function

（try_)borrow/borrow_mut

- 常用方法：

Rc内包含一个refcell