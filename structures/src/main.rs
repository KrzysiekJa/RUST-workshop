mod pear;

fn main()
{
    let color: pear::Color = pear::Color::Orange;
    let taste: pear::Taste = pear::Taste::Dry;
    let mut pear: pear::Pear = pear::Pear::new("Anjou Anjou".to_string(), color, 0.3, 0.2, taste);

    println!("pear: {:?}", pear);

    pear.get_variety();
    *pear.set_variety() = "Węgierka".to_string();

    pear.get_color();
    *pear.set_color() = pear::Color::Red;
    *pear.set_color() = pear::Color::Green;
    *pear.set_color() = pear::Color::Purple;

    pear.get_weight();
    *pear.set_weight() = 0.15;

    pear.get_size();
    *pear.set_size() = 0.1;

    pear.get_taste();
    *pear.set_taste() = pear::Taste::Sour;
    *pear.set_taste() = pear::Taste::Sweet;

    println!("pear: {:?}", pear);
}
