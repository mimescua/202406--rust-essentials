use csv::{ReaderBuilder, StringRecord};
use std::fs;
use std::collections::HashMap;
//use std::{fs, collections::HashMap};

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
struct DataHistory {
    datatype: String,
    tag: String,
    text: String,
    life: i32,
    options: Vec<DataHistory>,
}

impl DataHistory {
    fn new(row: StringRecord) -> DataHistory {
        let life = row.get(3).unwrap().trim();
        let life: i32 = life.parse().unwrap_or(0);

        return DataHistory {
            datatype: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life,
            options: vec![],
        };
    }
}

fn main() {
    let mut life = 100;
    let mut actual_tag = FIRST_TAG;

    let mut last_record: String = String::from("");

    // let mut data_history: Vec<DataHistory> = vec![];
    let mut data_history: HashMap<String, DataHistory> = HashMap::new();

    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let data = DataHistory::new(result);
        if data.datatype == "SITUACION" {
            let record_tag = data.tag.clone();

            // data_history.push(data);
            data_history.insert(data.tag.clone(), data);

            last_record = record_tag;
        }
        else if data.datatype == "OPCION" {
            if let Some(dat) = data_history.get_mut(&last_record) {
                (*dat).options.push(data);
            }
        }
    }

    //Game loop
    loop {
        println!("Tienes {} de vida", life);

        if let Some(dat) = data_history.get(actual_tag) {
            println!("{}", dat.text);

            for (index, option) in dat.options.iter().enumerate() {
                println!("[{}] {}", index, option.text);
            }

            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).unwrap();
            let selection = selection.trim().parse().unwrap_or(99);

            if let Some(selected) = &dat.options.get(selection) {
                actual_tag = &selected.tag;
            } else {
                println!("Comando no válido");
            }

            life += dat.life;
            println!("");
        } else  {
            break;
        }

        //Si la vida <= 0, entonces terminar juego
        if life <= 0 {
            println!("Has perdido!");
            break;
        }
    }

    // println!("{:?}", data_history);
}
