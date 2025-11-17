use anyhow::Result;
use serde::Serialize;
use strum::{
    Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, IntoEnumIterator,
    IntoStaticStr, VariantNames,
}; // ?? 什么作用

// 注入这么多宏是什么作用的?
#[allow(unused)]
#[derive(
    Debug, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, IntoStaticStr, VariantNames,
)]
enum MyEnum {
    A,
    B(String),
    C,
    D,
}

// strum_macros proc_macro Display
#[allow(unused)]
#[derive(Debug, Display, Serialize)] // 序列化宏
enum Color {
    #[strum(serialize = "redred", to_string = "red")]
    Red,
    Green {
        range: usize,
    },
    Blue(usize),
    Yellow,
    #[strum(to_string = "purple with {sat} saturation")]
    Purple {
        sat: usize,
    },
}

// ✅ cargo watch -x "run --package section04 --example errors_enum"
fn main() -> Result<()> {
    println!("errors_enum!");

    // 第一个枚举: MyEnum
    {
        println!("整体: {:?}", MyEnum::VARIANTS); // variants "实体/变体" --> 这是宏的功劳
        MyEnum::iter().for_each(|vari| println!("变体: {:?}", vari));
        println!("个数: {:?}", MyEnum::COUNT);

        // 创建一个枚举实例
        let my_enum: MyEnum = MyEnum::B("hello".to_string());
        println!(
            "{:?} {:?} {:?} {:?}",
            my_enum.is_a(),
            my_enum.is_b(),
            my_enum.is_c(),
            my_enum.is_d()
        );

        // 讲枚举变体实例转为字符串(core::convert::into)
        let my_enum_str: &'static str = my_enum.into();
        println!("{:?}", my_enum_str);
    }

    // 第二个枚举: Color
    {
        let red: Color = Color::Red;
        let green: Color = Color::Green { range: 10 };
        let blue: Color = Color::Blue(20);
        let yellow: Color = Color::Yellow;
        let purple: Color = Color::Purple { sat: 30 };

        println!(
            "red: {}, green: {}, blue: {}, yellow: {}, purple: {}",
            red, green, blue, yellow, purple
        );
        let red_str: String = serde_json::to_string(&red)?; // red枚举实例 --> 序列化为 "Red" 字符串
        println!("{}", red_str); // "Red"
        let green_str: String = serde_json::to_string(&green)?;
        println!("{}", green_str); // {"Green":{"range":10}}
        let yellow_str: String = serde_json::to_string(&yellow)?;
        println!("{}", yellow_str); // "Yellow"
    }

    Ok(())
}
