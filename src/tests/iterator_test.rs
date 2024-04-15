use crate::{Aggregade, Task, TaskList};



#[test]
fn iterator_test() {
    let mut task_list = TaskList::new();
    task_list.add_task(Task {id:1 ,description: "hacer el aseo".to_string() });
    task_list.add_task(Task {id:2, description: "lavar los platos ".to_string() });
    let mut iterator = task_list.get_iterator;


    assert_eq!(iterator.next().unwrap().description, "hacer el aseo");
    assert_eq!(iterator.next().unwrap().description, "lavar los platos ");
    assert!(iterator.next().is_none());

    
    
}

