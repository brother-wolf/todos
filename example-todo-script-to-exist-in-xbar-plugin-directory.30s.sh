#!/bin/bash

start_s=`/usr/local/bin/gdate +%s`
start_m=1`/usr/local/bin/gdate +%3N`

path_to_todos_binary=/Users/${USER}/path-to-todos-binary/todos
path_to_todo_list_symlink=/Users/${USER}/path-to-todo-list-symlink/.todo.md
path_to_markdown_editor=/Applications/Atom.app/Contents/MacOS/Atom

${path_to_todos_binary} --path ${path_to_todo_list_symlink} --icon inbox_tray --empty-icon zzz --editor ${path_to_markdown_editor} --format bitbar

end_s=`/usr/local/bin/gdate +%s`
end_m=1`/usr/local/bin/gdate +%3N`
diff_s=$((end_s - start_s))
add=$((diff * 1000))
diff_m=$((end_m - start_m))
render=$((add + diff_m))
echo "---"
echo "render time: $render ­ms"
echo "render finished: `/usr/local/bin/gdate -d @${end_s} +'%Y-%m-%d %H:%M:%S'`"