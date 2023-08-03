use std::cell::RefCell;
use std::rc::Rc;

pub fn smart_pointers(){
    let list: Vec<i32> = vec![1, 2, 3];

    println!("Smart Pointers");
    println!("Testing interior mutability with RefCell<Vec<i32>>");
    let mut_list = RefCell::new(list.clone());
    println!("{:?}", list);

    mut_list.borrow_mut().push(4);
    
    println!("After Update");
    println!("list {:?}", list);
    println!("mut_list {:?}", mut_list);
    println!("End test");


    println!("\n-----Rc-RefCell -----");
    let new_list = Rc::new(RefCell::new(vec![1, 2, 3]));

    let ref1 = Rc::clone(&new_list);
    let ref2 = Rc::clone(&new_list);

    println!("list before update {:?}", new_list);
    ref1.borrow_mut().push(6);
    ref2.borrow_mut().push(7);

    println!("list after update {:?}", new_list);


}