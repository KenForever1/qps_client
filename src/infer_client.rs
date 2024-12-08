
use std::io;

pub(crate) trait InferClient{
    fn infer(&mut self)-> io::Result<()>;
}