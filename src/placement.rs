use std::collections::BTreeMap;

use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, RectToInsert,
    TargetBin,
};

use crate::gamebox::GameBox;


#[derive(Debug, Hash, PartialEq, Eq, Clone, Ord, PartialOrd)]
enum MyCustomGroupId {
    GroupIdOne,
}



pub fn place_boxes(boxes: &Vec<GameBox>, max_width: u32, max_height: u32) -> Vec<Placement> {
    let rects_to_insert = map_rects_to_place(boxes);
    let rectangle_placements = find_placements(max_width, max_height, rects_to_insert);
    let packed_locations = rectangle_placements.packed_locations();

    let mut placements = vec![];
    for (game_name, (_id, pack)) in packed_locations {
        let game_box = boxes.iter().find(|b| &b.name == game_name).unwrap();
        placements.push(Placement {
            x: pack.x(),
            y: pack.y(),
            game: game_box,
        })
    }

    return placements;
}

pub struct Placement<'a> {
    pub x: u32,
    pub y: u32,
    pub game: &'a GameBox,
}

fn map_rects_to_place(boxes: &[GameBox]) -> GroupedRectsToPlace<String, MyCustomGroupId> {
    let mut rects_to_place = GroupedRectsToPlace::new();
    for game in boxes {
        rects_to_place.push_rect(
            game.name.clone(),
            Some(vec![MyCustomGroupId::GroupIdOne]),
            RectToInsert::new(
                (game.width + game.margin_right).ceil() as u32, //Ceil not round, as we would rather have a little too much margin than too little
                (game.height + game.margin_top).ceil() as u32,
                1,
            ),
        );
    }
    rects_to_place
}

fn find_placements(
    max_width: u32,
    max_height: u32,
    rects_to_place: GroupedRectsToPlace<String, MyCustomGroupId>,
) -> rectangle_pack::RectanglePackOk<String, i32> {
    let mut rectangle_placements = None;
    let mut width = max_width - 1;
    while rectangle_placements.is_none() {
        width = width + 1;
        let mut target_bins = BTreeMap::new();
        target_bins.insert(1, TargetBin::new(width, max_height, 1));

        rectangle_placements = pack_rects(
            &rects_to_place,
            &mut target_bins,
            &volume_heuristic,
            &contains_smallest_box,
        )
        .ok();
    }
    println!("W: {} , H: {}", width, max_height);
    rectangle_placements.unwrap()
}
