use idiomatic::LinkedList;

fn main() {
    println!("Hello, world!");
    let mut ll = LinkedList::new("vein");
    ll.append("for");
    ll.append("each");

    ll.iter().for_each(|item| println!("{}", item.borrow()))
}
