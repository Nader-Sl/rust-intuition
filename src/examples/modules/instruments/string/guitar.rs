//absolute path from crate root.
use crate::examples::modules::instruments::instrument::Instrument;

#[derive(Default, Debug)]
pub struct Guitar {}

 
impl Instrument for Guitar {

    fn play(&self) {
        println!("Playing {:?}", self);
    }
}