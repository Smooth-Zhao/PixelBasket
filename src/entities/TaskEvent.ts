type EventType = "task_start" | "task_running" | "task_failed" | "task_completed" | "task_canceled"
export default class TaskEvent {
  stage:EventType = "task_start"
  type = ""
  progress = 0
  data:any
}
