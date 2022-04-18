mod interaction;

pub trait Command {
    const NAME: &'static str;
    const DESC: &'static str;
}
