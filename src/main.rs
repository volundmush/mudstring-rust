use mudstring::core::{
    color::{Color, ColorSystem},
    style::{Style},
    text::{Segment, Span, Text}
};

fn main() {

    let mut style = Style::default();
    style.set_color(Color::from(1));
    let seg = Segment { text: "Hello red world!".to_string(), style: Some(style)};

    let mut style2 = Style::default();
    style2.set_color(Color::from(3));
    let seg2 = Segment {text: "Hello green world!".to_string(), style: Some(style2)};

    let mut v = Vec::with_capacity(3);
    v.push(seg);
    v.push(Segment {text: " ".to_string(), style: None});
    v.push(seg2);

    let t = Text::from(v);

    println!("Text is: {:?}", t);

    println!("{}", t.render(Some(ColorSystem::EightBit), false, false, false));

}
