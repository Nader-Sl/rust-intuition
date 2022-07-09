//absolute path from crate root.
use crate::examples::modules::instruments::instrument::Instrument;

#[derive(Default, Debug)]
pub struct Flute {}

 
impl Instrument for Flute {

    fn play(&self) {
        println!("Playing {:?}", self);
    }
    
}