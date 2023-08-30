use crate::task::TaskContext;

use super::CapPtr;

pub struct Task {
    start: usize,
    task_cx: TaskContext,
    next: Option<CapPtr>,
    next_task: Option<CapPtr>,
}
