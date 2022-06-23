//absolute path from crate root.
use crate::examples::modules::instruments::instrument::Instrument;

#[derive(Default, Debug)]
pub struct Cajon {}

 
impl Instrument for Cajon {

    fn play(&self) {
        println!("Playing {:?}", self);
    }
}