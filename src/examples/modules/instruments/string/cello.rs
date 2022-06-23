//absolute path from crate root.
use crate::examples::modules::instruments::instrument::Instrument;

#[derive(Default, Debug)]
pub struct Cello {}

 
impl Instrument for Cello {

    fn play(&self) {
        println!("Playing {:?}", self);
    }
}