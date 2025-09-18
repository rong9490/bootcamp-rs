fn main() {}

// TODO 补充书内容

#[derive(Debug)]
pub enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

// 条件编译
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut grains: Vec<Cereal> = vec![];
        grains.push(Cereal::Barley);
        grains.push(Cereal::Rye);
        grains.push(Cereal::Rice);
        grains.push(Cereal::Spelt);
        grains.push(Cereal::Wheat);

        assert_eq!(grains.len(), 4);
        // assert!(matches!(grains[0], Cereal::Barley));
        // assert!(matches!(grains[2], Cereal::Rice));
        // assert!(matches!(grains[4], Cereal::Wheat));

        println!("谷物列表: {:?}", grains);
    }
}
