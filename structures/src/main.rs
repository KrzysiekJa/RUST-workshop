mod pear;
mod engine;

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

    let mut engine: engine::Engine = engine::Engine::new(String::from("V8"), 20.0, 360.0);

    println!("engine: {:?}", engine);

    engine.get_mark();
    *engine.set_mark() = "V6".to_string();

    engine.get_capacity();
    *engine.set_capacity() = 18.0;

    engine.get_horsepower();
    *engine.set_horsepower() = 330.0;

    println!("engine: {:?}", engine);
    
}
