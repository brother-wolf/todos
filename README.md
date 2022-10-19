# ToDos

Used to parse a markdown file and output it for BitBar
A todo file should be saved in the format of `todo-${list_name}.md` and a symlink .todo.md created pointing to it.
The application scans this folder and gives the option to switch to another todo list by invoking `ln` via the terminal.


Markdown | bitbar interpretation
--- | ---
# | List Title
## | Grouping title
* [ ] ! | Task in progress
* [ ] | Task is todo
* [x] | Task is done
- | note
[name]\(url\) | if this is in a task it will be created as a link in a sub-task
\<indentation\> | if a task is indented below another task it will appear as a sub-task




