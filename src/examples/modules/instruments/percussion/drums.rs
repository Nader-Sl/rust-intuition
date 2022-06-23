//absolute path from crate root.
use crate::examples::modules::instruments::instrument::Instrument;

#[derive(Default, Debug)]
pub struct Drums {}

 
impl Instrument for Drums {

    fn play(&self) {
        println!("Playing {:?}", self);
    }
}