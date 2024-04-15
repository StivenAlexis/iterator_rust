//definimos la clase Task
pub struct Task {
    pub id: u32,
    pub description: String,

}

pub trait Aggregade {
    fn get_iterator(&self) -> TaskIterator;
    fn get_reverse_iterator(&self) -> ReverseTaskIterator;
    fn get_middle_to_left_iterator(&self) -> MiddleToLeftTaskIterator;
    fn get_middle_to_right_iterator(&self) -> MiddleToRightTaskIterator;
    
    
}
//definimos la clase TaskList
pub struct TaskList {
    tasks: Vec<Task>,
}


impl TaskList {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    
}

impl Aggregade for TaskList {
    
    fn get_iterator(&self) -> TaskIterator {
        TaskIterator { task_list: self, index: 0 }
    }

    fn get_reverse_iterator(&self) -> ReverseTaskIterator {
        ReverseTaskIterator { task_list: self, index: 0 }
    }

    fn get_middle_to_left_iterator(&self) -> MiddleToLeftTaskIterator {
        MiddleToLeftTaskIterator { task_list: self, index: 0 }
    }

    fn get_middle_to_right_iterator(&self) -> MiddleToRightTaskIterator {
        MiddleToRightTaskIterator { task_list: self, index: 0 }
    }
}



//definimos la clase TaskIterator para TaskList
pub struct TaskIterator<'a> {
    task_list: &'a TaskList,
    index: usize,
}

impl<'a> Iterator for TaskIterator<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.task_list.tasks.len() {
            let result = &self.task_list.tasks[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

// Define a new iterator that iterates over the tasks in reverse order
pub struct ReverseTaskIterator<'a> {
    task_list: &'a TaskList,
    index: usize,
}

impl<'a> Iterator for ReverseTaskIterator<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.task_list.tasks.len() {
            // Get the task from the end of the list
            let result = &self.task_list.tasks[self.task_list.tasks.len() - 1 - self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}


// Define a new iterator that iterates over the tasks from the middle to the left
pub struct MiddleToLeftTaskIterator<'a> {
    task_list: &'a TaskList,
    index: usize,
}

impl<'a> Iterator for MiddleToLeftTaskIterator<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        let middle = self.task_list.tasks.len() / 2;
        if self.index < middle {
            let result = &self.task_list.tasks[middle - 1 - self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

// Define a new iterator that iterates over the tasks from the middle to the right
pub struct MiddleToRightTaskIterator<'a> {
    task_list: &'a TaskList,
    index: usize,
}

impl<'a> Iterator for MiddleToRightTaskIterator<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        let middle = self.task_list.tasks.len() / 2;
        if self.index < middle {
            let result = &self.task_list.tasks[middle + 1 + self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests;
