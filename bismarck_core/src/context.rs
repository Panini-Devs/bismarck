use poise;
use crate::data::Data;
use crate::error::Error;

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type PartialContext<'a> = poise::PartialContext<'a, Data, Error>;