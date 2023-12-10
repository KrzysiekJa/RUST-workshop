#[derive(Debug, Copy, Clone)]
pub enum Color {
    Red,
    Green,
    Purple,
    Orange
}
#[derive(Debug, Copy, Clone)]
pub enum Taste {
    Sweet,
    Sour,
    Dry
}

#[derive(Debug, Clone)]
pub struct Pear {
    variety: String,
    color: Color,
    weight: f32,
    size: f32,
    taste: Taste
}

impl Pear 
{
    pub fn new (
        variety: String,
        color: Color,
        weight: f32,
        size: f32,
        taste: Taste
    ) -> Pear
    {
        return Pear {
            variety: variety,
            color: color,
            weight: weight,
            size: size,
            taste: taste
        };
    }

    pub fn get_variety( &self ) -> &str
    {
        return &self.variety;
    }

    pub fn set_variety( &mut self ) -> &mut String
    {
       return &mut self.variety;
    }
    
    pub fn get_color( &self ) -> Color
    {
        return self.color;
    }

    pub fn set_color( &mut self ) -> &mut Color
    {
       return &mut self.color;
    }

    pub fn get_weight( &self ) -> f32
    {
        return self.weight;
    }

    pub fn set_weight( &mut self ) -> &mut f32
    {
       return &mut self.weight;
    }

    pub fn get_size( &self ) -> f32
    {
        return self.size;
    }

    pub fn set_size( &mut self ) -> &mut f32
    {
       return &mut self.size;
    }

    pub fn get_taste( &self ) -> Taste
    {
        return self.taste;
    }

    pub fn set_taste( &mut self ) -> &mut Taste
    {
       return &mut self.taste;
    }
}
