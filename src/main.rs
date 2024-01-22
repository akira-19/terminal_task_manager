use clap::{Parser, Subcommand};
use rusqlite::Connection;
extern crate dirs;
use std::{error::Error, fs, path::PathBuf};

const TASK_MANAGER_DIR: &str = ".terminal_task_manager";
const DATABASE_FILE: &str = "tasks.db3";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, help = "Add a task")]
    add: Option<String>,

    #[arg(short, long, help = "Delete a task by id")]
    delete: Option<i32>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    #[clap(visible_alias("i"), about = "Initializes a task manager")]
    Init,
}

fn get_home_path() -> Result<PathBuf, Box<dyn Error>> {
    dirs::home_dir().ok_or_else(|| "Cannot get home dir".into())
}

fn create_dir(home_path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let path = home_path.join(TASK_MANAGER_DIR);
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

fn create_file(path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let path = path.join(DATABASE_FILE);
    if !path.exists() {
        fs::File::create(&path)?;
    }
    Ok(path)
}

fn init_dir() -> Result<PathBuf, Box<dyn Error>> {
    let home_path = get_home_path()?;
    let dir_path = create_dir(home_path)?;
    create_file(dir_path)
}

fn init_task_table(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
        )",
        (),
    )?;
    Ok(())
}

fn add_task(conn: &Connection, task: String) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "INSERT INTO tasks (task) VALUES (?)",
        rusqlite::params![task],
    )?;
    Ok(())
}

fn delete_task(conn: &Connection, task_id: i32) -> Result<(), Box<dyn Error>> {
    conn.execute("DELETE FROM tasks WHERE id = ?", rusqlite::params![task_id])?;
    Ok(())
}

fn list_tasks(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT id, task, created_at FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(format!(
            "{}. {}",
            row.get::<usize, i32>(0)?,
            row.get::<usize, String>(1)?
        ))
    })?;

    for task in task_iter {
        println!("{}", task?);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Init) => {
            let path = init_dir()?;
            let conn = Connection::open(&path)?;
            init_task_table(&conn)?;
            println!("Initialized task manager");
            return Ok(());
        }
        None => {}
    }

    let path = get_home_path()?.join(TASK_MANAGER_DIR).join(DATABASE_FILE);
    let conn = Connection::open(&path)?;

    if let Some(task) = cli.add {
        add_task(&conn, task)?;
    }

    if let Some(task_id) = cli.delete {
        delete_task(&conn, task_id)?;
    }

    list_tasks(&conn)?;

    Ok(())
}
