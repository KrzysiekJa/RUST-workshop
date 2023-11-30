mod pear;

fn main()
{
    let color: pear::Color = pear::Color::Orange;
    let taste: pear::Taste = pear::Taste::Dry;
    let pear: pear::Pear = pear::Pear::new("Anjou Anjou".to_string(), color, 0.3, 0.2, taste);

    println!("pear: {:?}", pear);
}
