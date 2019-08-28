use failure::Error;

pub trait System<'a> {

    type SystemInput;

    fn run(input : Self::SystemInput) -> Result<(), Error>;
}