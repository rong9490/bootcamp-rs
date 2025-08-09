#![allow(clippy::all, clippy::pedantic)]
// HACK rust-in-action 第一章

// fn main() {
//     greet_world();
// }

pub fn greet_world<'a>() -> [&'a str; 2] {
    let southern_germany: &'a str = "Grüß Gott!";
    let japan: &'a str = "ハロー・ワールド";
    let regions: [&'a str; 2] = [southern_germany, japan];
    regions
}

// fn cereals() -> () {
//     #[derive(Debug)]
//     enum Cereal {
//         Barley, Millet, Rice, // 元组形式
//         Rye, Spelt, Wheat,
//     }

//     let mut grains: Vec<Cereal> = vec![];
//     grains.push(Cereal::Barley);
//     grains.push(Cereal::Rye);
//     grains.push(Cereal::Rice);
//     grains.push(Cereal::Spelt);
//     grains.push(Cereal::Wheat);

//     println!("{:?}", grains);
// }

mod test {
    #[test]
    fn test_greet_world() {
        greet_world();

        // for region in regions.iter() {
        //     println!("{} {}", &region, region);
        // }
    }
}
