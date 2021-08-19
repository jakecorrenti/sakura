use rusqlite::{params, Connection, Result};
use uuid::Uuid;

const PATH: &str = "schedule.sql";

#[derive(Debug)]
struct ScheduleItem {
    id: Uuid,
    name: String,
    due_date: String,
    past_due: bool,
}

impl ScheduleItem {
    fn new(name: String, due_date: String) -> ScheduleItem {
        ScheduleItem {
            id: Uuid::new_v4(),
            past_due: false,
            name,
            due_date,
        }
    }
}

pub fn create_table() -> Result<()> {
    let conn = Connection::open(PATH)?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS schedule (
                id        TEXT PRIMARY KEY,
                name      TEXT NOT NULL,
                due_date  TEXT NOT NULL,
                past_due  BOOL
            )
        ",
        [],
    )?;

    Ok(())
}

pub fn save_new_item(name: &str, due_date: &str) -> Result<()> {
    let item = ScheduleItem::new(name.to_string(), due_date.to_string());
    println!("{:?}", item);

    let conn = Connection::open(PATH)?;

    conn.execute(
        "
            INSERT INTO schedule VALUES (?1, ?2, ?3, ?4)
        ",
        params![item.name, item.due_date, item.id.to_string(), item.past_due],
    )?;

    Ok(())
}
