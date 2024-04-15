use crate::{Aggregade, Task, TaskList};

#[test]
fn reverse_iterator_test() {
    let mut task_list = TaskList::new();
    task_list.add_task(Task { id:1,description: "Tarea 1".to_string() });
    task_list.add_task(Task { id:2, description: "Tarea 2".to_string() });

    let mut reverse_iterator = task_list.get_reverse_iterator();

    assert_eq!(reverse_iterator.next().unwrap().description, "Tarea 2");
    assert_eq!(reverse_iterator.next().unwrap().description, "Tarea 1");
    assert!(reverse_iterator.next().is_none());
}