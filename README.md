# terminal_task_manager

This is a super simple task manager working in the terminal.

# Requirement

This tool is using sqlite3.
Install sqlite3 If you have not installed yet.

# Install

You can install this tool via cargo.

```
cargo install ttm
```

# Usage

```
Usage: ttm [OPTIONS] [COMMAND]

Commands:
  init  Initializes a task manager [aliases: i]
  help  Print this message or the help of the given subcommand(s)

Options:
  -a, --add <ADD>        Add a task
  -d, --delete <DELETE>  Delete a task by id
  -h, --help             Print help
  -V, --version          Print version
```

First, you have to init the task manager.
When you init the task manager, <home path>/.terminal_task_manager/tasks.db3 will be created.

## Tips

just put 'ttm'. It shows the task list.
