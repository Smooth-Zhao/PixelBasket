import {reactive} from "vue";
import TaskEvent from "../entities/TaskEvent.ts";

const tasks = reactive<TaskEvent[]>([])
const useTask = () => {
  return {
    tasks,
    setTask(task: TaskEvent) {
      const index = tasks.findIndex(t => t.type === task.type)
      if (index !== -1){
        tasks[index] = task
      }else{
        tasks.push(task)
      }
    },
  }
}
export default useTask
