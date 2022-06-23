//absolute path from crate root.
use crate::examples::modules::instruments::instrument::Instrument;

#[derive(Default, Debug)]
pub struct Violin {}

 
impl Instrument for Violin {

    fn play(&self) {
        println!("Playing {:?}", self);
    }
}