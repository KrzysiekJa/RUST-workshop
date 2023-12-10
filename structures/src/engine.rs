#[derive(Debug, Clone)]
pub struct Engine {
    mark: String,
    capacity: f32,
    horsepower: f32
}

impl Engine 
{
    pub fn new (
        mark: String,
        capacity: f32,
        horsepower: f32
    ) -> Engine
    {
        return Engine {
            mark: mark,
            capacity: capacity,
            horsepower: horsepower
        };
    }

    pub fn get_mark( &self ) -> String
    {
        return self.mark.clone();
    }

    pub fn set_mark( &mut self ) -> &mut String
    {
       return &mut self.mark;
    }

    pub fn get_capacity( &self ) -> f32
    {
        return self.capacity;
    }

    pub fn set_capacity( &mut self ) -> &mut f32
    {
       return &mut self.capacity;
    }

    pub fn get_horsepower( &self ) -> f32
    {
        return self.horsepower;
    }

    pub fn set_horsepower( &mut self ) -> &mut f32
    {
       return &mut self.horsepower;
    }
}
