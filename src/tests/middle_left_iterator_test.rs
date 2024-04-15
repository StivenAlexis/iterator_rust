
use crate::Aggregade;
use crate::TaskList;
use crate::Task;

#[test]
fn middle_to_left_iterator_test() {
    let mut task_list = TaskList::new();
    task_list.add_task(Task {id:1, description: "Tarea 1".to_string() });
    task_list.add_task(Task {id:2, description: "Tarea 2".to_string() });
    task_list.add_task(Task {id:3, description: "Tarea 3".to_string() });


    let mut middle_to_left_iterator = task_list.get_middle_to_left_iterator();

    assert_eq!(middle_to_left_iterator.next().unwrap().description, "Tarea 1");
    assert!(middle_to_left_iterator.next().is_none());
}
