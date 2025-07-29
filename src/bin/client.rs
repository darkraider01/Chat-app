//importing the various modules from .cursive library for UI dev
use Cursive::{
    align::HAlign,
    event::Key,
    theme::{BorderStyle, Color, Palette, PaletteColor, Theme, BaseColor},
    traits::*,
    views::{Dialog, DummyView, EditView, LinearLayout, Panel, ScrollView, TextView},
    Cursive,
};
use tokio:: {
    net::TcpStream,
    sync::Mutex,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use serde::{Serialize, Deserialize};
use std::{env,error::Error, sync::Arc}; // Arc will be useful for sharing state




#[tokio::main]
async fn main()  {
    // Client implementation will go here

    
}


//Create the retro style theme function