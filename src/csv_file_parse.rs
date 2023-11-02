use serde::Deserialize;

use crate::gamebox::GameBox;

#[derive(Deserialize)]
struct CsvBox {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Width cm")]
    width: f32,

    #[serde(rename = "Height cm")]
    height: f32,
}

impl Into<GameBox> for CsvBox {
    fn into(self) -> GameBox {
        let b = self;
        let margin_top = (b.height / 8.) + 3.;
        let margin_right = 2.;
        GameBox {
            height: b.height,
            name: b.name.to_owned(),
            width: b.width,
            margin_top,
            margin_right,
        }
    }
}

pub fn read_games_from_input(path: &str) -> Vec<GameBox> {
    let csv_data = std::fs::read_to_string(path).unwrap();
    let mut boxes: Vec<GameBox> = csv::Reader::from_reader(csv_data.as_bytes())
        .deserialize()
        .flat_map(|r| r.ok())
        .map(|csvbox: CsvBox| csvbox.into())
        .collect();
    boxes.sort_by_key(|a| (a.width * a.height) as u32);
    boxes.reverse();
    boxes
}
