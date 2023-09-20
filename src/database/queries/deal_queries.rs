use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::models;
use self::models::{Deal, NewDeal};
use serde::{Serialize, Deserialize};
use crate::lib::establish_connection;