use linked_list::LinkedList;

use crate::linked_list::ComputeNorm;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<String> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        let string = String::from(format!("{}", i));
        list.push_front(string);
    }

    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    let mut list1 = list.clone();
    println!("{}", &list1);
    let mut is_eq = list == list1;
    println!("{}", is_eq);
    let eat = list1.pop_front();
    is_eq = list == list1;
    println!("{}", is_eq);


    println!("Testing norm");
    let mut arr: LinkedList<f64> = LinkedList::new();
    arr.push_front(3.0);
    arr.push_front(4.0);
    println!("{}", arr.compute_norm());


    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
