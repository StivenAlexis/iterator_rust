use crate::{Aggregade, Task, TaskList};



#[test]
fn iterator_test() {
    let mut task_list = TaskList::new();
    task_list.add_task(Task {id:1 ,description: "hacer el aseo".to_string() });
    task_list.add_task(Task {id:2, description: "lavar los platos ".to_string() });
    let mut iterator = task_list.get_iterator();


    assert_eq!(iterator.next().unwrap().description, "hacer el aseo");
    assert_eq!(iterator.next().unwrap().description, "lavar los platos ");
    assert!(iterator.next().is_none());

    
    
}

#[test]
fn iterator_intensive_test() {
    let mut task_list = TaskList::new();
    for i in 0..1000 {
    task_list.add_task(Task {id:i, description: "".to_string() });
    
    }
    let mut iterator = task_list.get_iterator();
    let mut id_expected = 0;
    
    while let Some(task) = iterator.next() {
       
        assert_eq!(task.id, id_expected);
        id_expected= id_expected + 1;
    }
}

