use vglyph::compile;

fn main() {
    let input = include_str!("../res/v.glyph");
    let map = compile(input);

    dbg!(map.iter().filter(|(_, (input, _))| input.eq(&["human", "hands", "question"])).take(1).collect::<Vec<_>>());
}

