use rusqlite::{params, Connection, NO_PARAMS,};
use std::fs::{File, self};
use anyhow::Result;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open("/Users/tenx/desktop/database_test/init.db")?;
    let me = Person {
        id: 3,
        name: "Kallen".to_string(),
        data: Some("你好👋".as_bytes().to_vec()),
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let mut rows = stmt.query(NO_PARAMS)?;
    while let Some(row) = rows.next()? {
        println!("Found person {:?}, id: {:?}, data: {:?}",
                 row.get::<_, String>(1)?,
                 row.get::<_, i32>(0)?,
                 String::from_utf8(row.get::<_, Option<Vec<u8>>>(2)?.unwrap())?
        );
    }

    // let person_iter = stmt.query_map(NO_PARAMS, |row| {
    //     Ok(Person {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //         data: row.get(2)?,
    //     })
    // })?;
    //
    // for person in person_iter {
    //     let p = person.unwrap();
    //     println!("Found person {:?}, id: {}, data: {}",
    //              p.name, p.id, String::from_utf8(p.data.clone().unwrap())?);
    // }
    Ok(())
}
