A simple todolist implementation using rust programming language.

Usage: tdl [COMMAND] [ARGUMENTS]
commands:
    - add [TASK/s]
        adds new task/s
        Example: tdl add workout read sleep
    - list
        lists all tasks
        Example: tdl list
    - done [INDEX/s]
        marks task/s as done
        Example: tdl done 4 5 6  (marks 4th, 5th and 6th tasks as completed)
    - rm [INDEX/s]
        removes a task/s
        Example: todo rm 4 (renoves the fourth task on the list)
    - reset
        deletes all tasks
