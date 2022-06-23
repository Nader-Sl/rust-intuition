//absolute path from crate root.
use crate::examples::modules::instruments::instrument::Instrument;

#[derive(Default, Debug)]
pub struct Trumpet {}

 
impl Instrument for Trumpet {

    fn play(&self) {
        println!("Playing {:?}", self);
    }
}