use rusqlite::{params, Connection, Result};
use uuid::Uuid;

const PATH: &str = "schedule.sql";

#[derive(Debug)]
pub struct ScheduleItem {
    id: Uuid,
    name: String,
    due_date: String,
    past_due: bool,
}

impl ScheduleItem {
    fn new(name: String, due_date: String) -> ScheduleItem {
        ScheduleItem {
            id: Uuid::new_v4(),
            name,
            due_date,
            past_due: false,
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
        "INSERT INTO schedule VALUES (?1, ?2, ?3, ?4)",
        params![item.id.to_string(), item.name, item.due_date, item.past_due],
    )?;

    Ok(())
}

pub fn get_all_items() -> Result<Vec<ScheduleItem>, rusqlite::Error> {
    let conn = Connection::open(PATH)?;

    let mut statement = conn.prepare("SELECT id, name, due_date, past_due FROM schedule")?;
    let iter = statement.query_map([], |row| {
        let id: String = row.get(0)?;
        Ok(ScheduleItem {
            id: Uuid::parse_str(id.as_str()).unwrap(),
            name: row.get(1)?,
            due_date: row.get(2)?,
            past_due: row.get(3)?,
        })
    })?;

    let mut items = Vec::new(); 
    for ii in iter {
        items.push(ii?); 
    }

    Ok(items)
}
