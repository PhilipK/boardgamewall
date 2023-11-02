use boundries::get_outer_boundries_from_args;
use csv_file_parse::read_games_from_input;
use placement::place_boxes;
use svg_generation::generate_svg;

mod gamebox;
mod boundries;
mod csv_file_parse;
mod placement;
mod svg_generation;
mod box_to_placement;

pub fn main() {
    let boxes = read_games_from_input("bgg.csv");
    let (max_height, max_width) = get_outer_boundries_from_args();
    let place_boxes = place_boxes(&boxes, max_width, max_height);
    let svg = generate_svg(place_boxes);
    std::fs::write("out.svg", svg).unwrap();
}

