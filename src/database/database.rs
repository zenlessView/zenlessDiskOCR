use {
  crate::disk::disk::{
    Affix,
    Disk
  },
  sqlite::{Connection, ConnectionThreadSafe, State, Statement, Value},
  std::collections::HashMap,
  tokio::sync::OnceCell
};

static CONNECTION: OnceCell<Result<ConnectionThreadSafe, String>> = OnceCell::const_new();

async fn get_connection() -> &'static Result<ConnectionThreadSafe, String> {
  CONNECTION
    .get_or_init(|| {
      async {
        let connection = Connection::open_thread_safe("database.db");
        match connection {
          Ok(connection) => Ok(connection),
          Err(error) => Err(format!("Failed to open database: {}", error))
        }
      }
    })
    .await
}

async fn init_database() -> Result<(), String> {
  let connection = match get_connection().await {
    Ok(connection) => connection,
    Err(error) => return Err(error.clone())
  };

  let table_stat = connection
    .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='disks'")
    .map_err(|error| format!("Failed to prepare statement: {}", error))?
    .next();

  if let Ok(State::Done) = table_stat {
    match connection.execute(
      r#"
        CREATE TABLE disks (
          id INTEGER AUTOINCREMENT PRIMARY KEY,
          set TEXT NOT NULL,
          slot INTEGER NOT NULL,
          primary_affix_name TEXT NOT NULL,
          primary_affix_value REAL NOT NULL,
          primary_affix_value_type TEXT NOT NULL,
          secondary_affix_1_name TEXT,
          secondary_affix_1_value REAL,
          secondary_affix_1_value_type TEXT,
          secondary_affix_2_name TEXT,
          secondary_affix_2_value REAL,
          secondary_affix_2_value_type TEXT,
          secondary_affix_3_name TEXT,
          secondary_affix_3_value REAL,
          secondary_affix_3_value_type TEXT,
          secondary_affix_4_name TEXT,
          secondary_affix_4_value REAL,
          secondary_affix_4_value_type TEXT
        );
        "#
    ) {
      Ok(_) => Ok(()),
      Err(error) => Err(format!("Failed to create table: {}", error))
    }
  } else {
    Ok(())
  }
}

async fn insert_disk(disk: &Disk) -> Result<(), String> {
  let connection = match get_connection().await {
    Ok(connection) => connection,
    Err(error) => return Err(error.clone())
  };

  let mut statement = match connection.prepare(
    r#"
    INSERT INTO disks (
      set,
      slot,
      primary_affix_name,
      primary_affix_value,
      primary_affix_value_type,
      secondary_affix_1_name,
      secondary_affix_1_value,
      secondary_affix_1_value_type,
      secondary_affix_2_name,
      secondary_affix_2_value,
      secondary_affix_2_value_type,
      secondary_affix_3_name,
      secondary_affix_3_value,
      secondary_affix_3_value_type,
      secondary_affix_4_name,
      secondary_affix_4_value,
      secondary_affix_4_value_type
    )
    VALUES (
      set = :set,
      slot = :slot,
      primary_affix_name = :primary_affix_name,
      primary_affix_value = :primary_affix_value,
      primary_affix_value_type = :primary_affix_value_type,
      secondary_affix_1_name = :secondary_affix_1_name,
      secondary_affix_1_value = :secondary_affix_1_value,
      secondary_affix_1_value_type = :secondary_affix_1_value_type,
      secondary_affix_2_name = :secondary_affix_2_name,
      secondary_affix_2_value = :secondary_affix_2_value,
      secondary_affix_2_value_type = :secondary_affix_2_value_type,
      secondary_affix_3_name = :secondary_affix_3_name,
      secondary_affix_3_value = :secondary_affix_3_value,
      secondary_affix_3_value_type = :secondary_affix_3_value_type,
      secondary_affix_4_name = :secondary_affix_4_name,
      secondary_affix_4_value = :secondary_affix_4_value,
      secondary_affix_4_value_type = :secondary_affix_4_value_type
    );
    "#
  ) {
    Ok(statement) => statement,
    Err(error) => return Err(format!("Failed to prepare statement: {}", error))
  };

  let affix_1 = disk.secondary_affixes[0].as_ref();
  let affix_2 = disk.secondary_affixes[1].as_ref();
  let affix_3 = disk.secondary_affixes[2].as_ref();
  let affix_4 = disk.secondary_affixes[3].as_ref();

  match statement.bind::<&[(_, Value)]>(
    &[
      (":set", Value::String(disk.set.to_string())),
      (":slot", Value::Integer(i64::from(disk.slot))),
      (":primary_affix_name", Value::String(disk.primary_affix.name.to_string())),
      (":primary_affix_value", Value::Float(disk.primary_affix.value)),
      (
        ":primary_affix_value_type",
        Value::String(disk.primary_affix.value_type.to_string())
      ),
      (
        ":secondary_affix_1_name",
        if let Some(affix) = affix_1 {
          Value::String(affix.name.to_string())
        } else {
          Value::Null
        }
      ),
      (
        ":secondary_affix_1_value",
        if let Some(affix) = affix_1 { Value::Float(affix.value) } else { Value::Null }
      ),
      (
        ":secondary_affix_1_value_type",
        if let Some(affix) = affix_1 {
          Value::String(affix.value_type.to_string())
        } else {
          Value::Null
        }
      ),
      (
        ":secondary_affix_2_name",
        if let Some(affix) = affix_2 {
          Value::String(affix.name.to_string())
        } else {
          Value::Null
        }
      ),
      (
        ":secondary_affix_2_value",
        if let Some(affix) = affix_2 { Value::Float(affix.value) } else { Value::Null }
      ),
      (
        ":secondary_affix_2_value_type",
        if let Some(affix) = affix_2 {
          Value::String(affix.value_type.to_string())
        } else {
          Value::Null
        }
      ),
      (
        ":secondary_affix_3_name",
        if let Some(affix) = affix_3 {
          Value::String(affix.name.to_string())
        } else {
          Value::Null
        }
      ),
      (
        ":secondary_affix_3_value",
        if let Some(affix) = affix_3 { Value::Float(affix.value) } else { Value::Null }
      ),
      (
        ":secondary_affix_3_value_type",
        if let Some(affix) = affix_3 {
          Value::String(affix.value_type.to_string())
        } else {
          Value::Null
        }
      ),
      (
        ":secondary_affix_4_name",
        if let Some(affix) = affix_4 {
          Value::String(affix.name.to_string())
        } else {
          Value::Null
        }
      ),
      (
        ":secondary_affix_4_value",
        if let Some(affix) = affix_4 { Value::Float(affix.value) } else { Value::Null }
      ),
      (
        ":secondary_affix_4_value_type",
        if let Some(affix) = affix_4 {
          Value::String(affix.value_type.to_string())
        } else {
          Value::Null
        }
      )
    ][..]
  ) {
    Ok(_) => (),
    Err(error) => return Err(format!("Failed to bind set: {}", error))
  }

  match statement.next() {
    Ok(State::Done) => Ok(()),
    Ok(State::Row) => Err("Unexpected row".to_string()),
    Err(error) => Err(format!("Failed to execute statement: {}", error))
  }
}

async fn read_disk() -> Result<HashMap<i64, Disk>, String> {
  let connection = match get_connection().await {
    Ok(connection) => connection,
    Err(error) => return Err(error.clone())
  };

  let table_stat = connection
    .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='disks'")
    .map_err(|error| format!("Failed to prepare statement: {}", error))?
    .next();

  if let Ok(State::Done) = table_stat {
    return Err("Table does not exist".to_string());
  }

  let mut statement = match connection.prepare("SELECT * FROM disks") {
    Ok(statement) => statement,
    Err(error) => return Err(format!("Failed to prepare statement: {}", error))
  };

  let mut result: HashMap<i64, Disk> = HashMap::new();

  while let Ok(State::Row) = statement.next() {
    let id: i64 = match statement.read::<i64, _>("id") {
      Ok(id) => id,
      Err(error) => return Err(format!("Failed to read id: {}", error))
    };

    let set: &'static str = match statement.read::<String, _>("set") {
      Ok(set) => {
        match Disk::get_static_set_name(&set) {
          Some(set) => set,
          None => return Err(format!("Invalid disk set: {}", set))
        }
      },
      Err(error) => return Err(format!("Failed to read set: {}", error))
    };

    let slot: u8 = match statement.read::<i64, _>("slot") {
      Ok(slot) => {
        if slot > 0 && slot < 7 {
          slot as u8
        } else {
          return Err(format!("Invalid slot: {}", slot));
        }
      },
      Err(error) => return Err(format!("Failed to read slot: {}", error))
    };

    let primary_affix = if let (Ok(name), Ok(value), Ok(value_type)) = (
      statement.read::<String, _>("primary_affix_name"),
      statement.read::<f64, _>("primary_affix_value"),
      statement.read::<String, _>("primary_affix_value_type")
    ) {
      Affix::try_from((&name as &str, value, &value_type as &str))?
    } else {
      return Err("Failed to read primary affix".to_string());
    };

    let mut secondary_affixes: [Option<Affix>; 4] = [None, None, None, None];

    fn read_secondary_affixes_and_save_if_fail(
      statement: &mut Statement,
      secondary_affixes: &mut [Option<Affix>; 4],
      index: usize,
      env: (i64, &str, u8, &Affix, &mut HashMap<i64, Disk>)
    ) -> Result<(), String> {
      if index > 3 {
        return Err("Invalid index".to_string());
      }

      let name_key = format!("secondary_affix_{}_name", index + 1);
      let value_key = format!("secondary_affix_{}_value", index + 1);
      let value_type_key = format!("secondary_affix_{}_value_type", index + 1);

      if let (Ok(Some(name)), Ok(Some(value)), Ok(Some(value_type))) = (
        statement.read::<Option<String>, _>(&name_key as &str),
        statement.read::<Option<f64>, _>(&value_key as &str),
        statement.read::<Option<String>, _>(&value_type_key as &str)
      ) {
        let affix = Affix::try_from((&name as &str, value, &value_type as &str))?;

        secondary_affixes[index] = Some(affix);

        Ok(())
      } else {
        let disk = Disk::new(
          env.1,
          env.2,
          env.3.clone(),
          secondary_affixes[0].take(),
          secondary_affixes[1].take(),
          secondary_affixes[2].take(),
          secondary_affixes[3].take()
        )?;

        env.4.insert(env.0, disk);

        return Err("no more secondary affixes".to_string());
      }
    }

    for i in 0 .. 4 {
      if let Err(error) = read_secondary_affixes_and_save_if_fail(
        &mut statement,
        &mut secondary_affixes,
        i,
        (id, set, slot, &primary_affix, &mut result)
      ) {
        if error == "no more secondary affixes" {
          break;
        } else {
          return Err(error);
        }
      }
    }
  }

  Ok(result)
}
