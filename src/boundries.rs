pub fn get_outer_boundries_from_args() -> (u32, u32) {
    let mut args = std::env::args();
    args.next(); //First arg is the program name
    let max_height: u32 = args.next().unwrap_or_default().parse().unwrap_or(100);
    let max_width: u32 = args.next().unwrap_or_default().parse().unwrap_or(0);
    (max_height, max_width)
}