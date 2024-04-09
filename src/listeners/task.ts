import {listen} from "@tauri-apps/api/event";
import TaskEvent from "../entities/TaskEvent.ts";

const initTaskEventListener = () => {
  listen<TaskEvent>("task", event => {
    taskEventHandler(event.payload)
  }).then(() => {})
}

const taskEventHandler = (event: TaskEvent) => {
  switch (event.stage) {
    case "task_start":
  }
}

export default initTaskEventListener;
