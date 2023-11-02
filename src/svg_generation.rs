pub struct BoxDraw<'a> {
    pub x: f32,
    pub y: f32,
    pub name: &'a str,
    pub width: f32,
    pub height: f32,
}

pub fn generate_svg<'a>(boxes: Vec<impl Into<BoxDraw<'a>>>) -> String {
    let boxes: Vec<BoxDraw> = boxes.into_iter().map(|p| (p).into()).collect();
    let scale_up = 5.;
    let mut svg_content = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\">\n",
    );

    for box_draw in boxes {
        let rect = format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" stroke=\"black\" stroke-width=\"2\" fill=\"none\"/>\n",
            box_draw.x* scale_up, 
            box_draw.y* scale_up,
            box_draw.width* scale_up,
            box_draw.height* scale_up
        );
        svg_content.push_str(&rect);

        let text_x = box_draw.x * scale_up + 5.;
        let text_y = box_draw.y * scale_up + 15.;

        let text = format!(
            "<text x=\"{}\" y=\"{}\" font-family=\"Verdana\" font-size=\"12\">{}</text>\n",
            text_x, text_y, box_draw.name
        );
        svg_content.push_str(&text);
    }

    svg_content.push_str("</svg>");
    svg_content
}
