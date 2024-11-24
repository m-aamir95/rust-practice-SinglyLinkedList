use std::fmt;
use std::fmt::Formatter;

struct Node{
    value : i32,
    next : Option<Box<Node>>
}

impl fmt::Display for Node{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)?;
        Ok(())
    }
}

struct SinglyLinkedList{
    head : Option<Box<Node>>
}

impl SinglyLinkedList{

    fn new() -> Self{
        Self { head : None}
    }

    fn push(&mut self, value : i32){

        let new_node = Box::new(
            Node{
                value,
                next : self.head.take()
            }
        );
        self.head = Some(new_node);
    }

    fn search(&self, value : i32) -> bool{
        let mut current = &self.head;
        while let Some(node) = current {
            if node.value == value{
                return true;
            }
            current = &node.next;
        }
        false
    }

    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current{
            println!("{}", node);
            current = &node.next;
        }
    }
}

fn main() {

    let mut list = SinglyLinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);

    list.print();

    println!("\n");
    search_num_in_list(2, &list);
    search_num_in_list(10, &list);
}

fn search_num_in_list(value_to_search : i32, list : &SinglyLinkedList){
    if list.search(value_to_search) {
        println!("{}, Exists in the list", value_to_search);
    }else{

        println!("{}, Does not Exists in the list", value_to_search);
    }
}