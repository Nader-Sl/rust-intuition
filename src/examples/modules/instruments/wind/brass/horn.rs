//absolute path from crate root.
use crate::examples::modules::instruments::instrument::Instrument;

#[derive(Default, Debug)]
pub struct Horn {}

 
impl Instrument for Horn {

    fn play(&self) {
        println!("Playing {:?}", self);
    }
}