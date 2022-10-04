# Intro to `Rust`

## Difference between `C/C++` and Rust

- C/C++

C调用函数是进行压栈操作

当函数完成调用之后，把局部变量弹出

内存栈当中局部变量地方可以作为缓存，存放你将要读取的数据

如果读取的内容**超过缓存区**，会出现以下几种问题

假设一个简单的栈长成这个样子：

... | function addr | return address | base ptr | local variables | ...



1.如果读取内容超过整个内存栈，会出现栈溢出现象，程序会崩溃报错（crash）

2.如果读取的内容覆盖了局部变量缓存区，比如说覆盖了返回数地址，程序不会报错，但是不知道要返回什么值，甚至可以把前面的内容**恶意利用**（比如说栈攻击，类似于蠕虫病毒），例如c语言当中的`gets()`函数，`strncpy()`函数（最后一个参数是unsigned，会有cast，如果cast -1 就会出问题）

这些问题很难用工具找到，因为不知道什么时候会出问题（程序运行正常，但是他**有出错的可能**） 

- Garbage Collection Language

不用再考虑`allocate`的问题 

“Disruptive", drop what you are doing (time for GC)

"Not Deterministic": when will the next GC pause be?

有的时候要控制内存来达到数据结构的高效，但是GC不知道，只能给你用一般的方法

GC也会存在内存方面的问题

  

