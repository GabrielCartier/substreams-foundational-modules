use anyhow::Error;

use crate::pb::sf::substreams::ethereum::v1::{Call, Calls};
use prost::Message;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};

use substreams_ethereum::pb::eth::v2::Block;

use base64::decode;
use std::fs;

pub mod calls;
pub mod combined;
pub mod events;
pub mod pb;

substreams_ethereum::init!();
