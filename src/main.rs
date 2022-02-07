use rgb2ansi256::rgb_to_ansi256;

fn main() {
    println!("Hello, world!");
    let colors = vec!["255;74;68", "122;112;112", "102;217;239"];
    for c in colors {
        let sp: Vec<&str> = c.split(";").collect();
        let r = sp[0].parse::<u8>().unwrap();
        let g = sp[1].parse::<u8>().unwrap();
        let b = sp[2].parse::<u8>().unwrap();
        let co = rgb_to_ansi256(r, g, b);
        println!("c: {} co: {}", c, co);
    }
}
