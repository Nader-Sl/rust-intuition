//absolute path from crate root.
use crate::examples::modules::instruments::instrument::Instrument;

#[derive(Default, Debug)]
pub struct Clarinet {}

 
impl Instrument for Clarinet {

    fn play(&self) {
        println!("Playing {:?}", self);
    }
}