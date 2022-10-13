# Lecture 4: Object Oriented Rust

## Struct

- 结构题是满足Type safety的

- allocate

Box(类似于unique ptr),会进行auto free（使用drop函数）
```rust
// Showing Box
fn main(){
  let x: Box<u32> = Box::new(10);
}

```
### 面向对象的单链表示例

```rust
struct LinkedList {
  head: Option<Box<Node>>, //因为链表不一定有第一个元素，可能是None
  size: usize,
}
	
struct Node {
  value: u32,
  next: Option<Box<Node>>, // Wrong situation: Box<&Node> 因为没有借用者
}

impl Node {
  pub fn new(value: u32, next: Option<Box<Node>>) -> Node {
    Node {value: value, next: next}// 没有 ; 因为要返回一个Node
  }
}

impl LinkedList {
  pub fn new() -> LinkedList {
    LinkedList {head: None, size: 0}
  }
  
  pub fn get_size(&self) -> usize {
    self.size // 使用成员变量的时候不用解引用，rust的语法糖能够让自引用的时候尽可能的解引用
  }
  
  pub fn is_empty(&self) -> bool {
    head.is_none()
    // self.size == 0
    // self.get_size() == 0
  }
  
  pub fn push(&mut self, value: u32) {
    //let new_node: Box<Node> = Box::new(Node::new(value, self.head)); 不符合ownership的条件，出错
    //正确写法: 使用take()方法 (unsafe rust)
    let new_node: Option<Box<Node>> = Some(Box::new(Node::new(value, self.head.take())));
    self.head = Some(new_node);
    self.size += 1;
  }
  
  pub fn pop(&mut self) -> Option<u32>{
    // 精华全在代码里
    let node: Box<Node> = self.head.take()?;
    self.head = node.next;
    self.size -= 1;
    Some(node.value)
  }
}

fn main() {
  let list: LinkedList = LinkedList::new();
  assert!(list.is_empty());
  assert!(list.get_size(), 0);
}
```



