mod linkedlist;
use linkedlist::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push("apple");
    list.push("banana");
    list.push("carrot");
    println!("should be carrot: {:?}", list.pop()); 
    println!("should be banana: {:?}", list.pop()); 
    println!("should be apple: {:?}", list.pop()); 
}
