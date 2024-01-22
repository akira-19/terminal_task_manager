use clap::{Parser, Subcommand};
use rusqlite::Connection;
extern crate dirs;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, help = "Add a task")]
    add: Option<String>,

    #[arg(short, long, help = "delete a task")]
    delete: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    #[clap(visible_alias("i"), about = "initializes a task manager")]
    Init,
}

fn get_home_path() -> Result<PathBuf, String> {
    if let Some(path) = dirs::home_dir() {
        Ok(path)
    } else {
        Err("Cannot get home dir".to_string())
    }
}

fn create_dir(home_path: PathBuf) -> Result<PathBuf, String> {
    let path = home_path.join(".terminal_task_manager");

    match fs::create_dir_all(path.clone()) {
        Err(_) => Err("Unable to create directory".to_string()),
        Ok(_) => Ok(path),
    }
}

fn create_file(path: PathBuf) -> Result<PathBuf, String> {
    let path = path.join("tasks.db3");

    match fs::File::create(path.clone()) {
        Err(_) => Err("Unable to create file".to_string()),
        Ok(_) => Ok(path),
    }
}

fn init_dir() -> Result<PathBuf, String> {
    let home_path = get_home_path()?;
    let path = create_dir(home_path)?;
    create_file(path)
}

fn init_task_table(conn: &Connection) -> Result<(), String> {
    match conn.execute(
        "CREATE TABLE tasks (
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            task  TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
        )",
        (),
    ) {
        Err(_) => Err("Unable to create table".to_string()),
        Ok(_) => Ok(()),
    }
}

fn add_task(conn: &Connection, task: String) -> Result<(), String> {
    match conn.execute(
        "INSERT INTO tasks (task) VALUES (?)",
        rusqlite::params![task],
    ) {
        Err(_) => Err("Unable to add task".to_string()),
        Ok(_) => Ok(()),
    }
}

fn delete_task(conn: &Connection, task: String) -> Result<(), String> {
    match conn.execute("DELETE FROM tasks WHERE id = ?", rusqlite::params![task]) {
        Err(_) => Err("Unable to delete task".to_string()),
        Ok(_) => Ok(()),
    }
}

fn list_tasks(conn: &Connection) -> Result<(), String> {
    let mut stmt = match conn.prepare("SELECT id, task, created_at FROM tasks") {
        Err(_) => return Err("Unable to prepare statement".to_string()),
        Ok(stmt) => stmt,
    };

    let task_iter = match stmt.query_map([], |row| {
        Ok(format!(
            "{}. {} - {}",
            row.get::<usize, i32>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?
        ))
    }) {
        Err(_) => return Err("Unable to query map".to_string()),
        Ok(task_iter) => task_iter,
    };

    for task in task_iter {
        println!("{}", task.unwrap());
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Init) => {
            let path = init_dir().expect("Unable to create directory");
            let conn = Connection::open(&path).unwrap();
            init_task_table(&conn).expect("Unable to create table");
            println!("Initialized task manager");
            return;
        }
        None => {}
    }

    let path = get_home_path().expect("Unable to get home dir");
    let path = path.join(".terminal_task_manager");
    let path = path.join("tasks.db3");
    let conn = Connection::open(&path).unwrap();

    if let Some(v) = cli.add {
        add_task(&conn, v).expect("Unable to add task");
    }

    if let Some(v) = cli.delete {
        delete_task(&conn, v).expect("Unable to delete task");
    }

    list_tasks(&conn).expect("Unable to list tasks");
}
