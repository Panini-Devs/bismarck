use crate::data::Data;
use crate::error::Error;
use poise;

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type PartialContext<'a> = poise::PartialContext<'a, Data, Error>;
