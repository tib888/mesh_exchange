use crate::content::{Mesh, Position};

#[derive(Debug)]
pub struct WhereIs
{    
}

impl WhereIs
{
    pub fn new<N>(mesh: Mesh<N>) -> Self
    {        
        WhereIs {
            //TODO
        }
    }

    pub fn is_inside<N>(&self, point: &Position) -> bool
    {
        // TODO ? hataron levo pontokkal mi legyen? kint / bent?
        false
    }

    //TODO implementation for many points?
}
