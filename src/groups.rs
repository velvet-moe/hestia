use crate::commands::dev::shutdown::*;
use crate::commands::fun::{
    cat::*, coinflip::*, dadjoke::*, dog::*, eightball::*, roll::*, waifu::*,
};
use crate::commands::misc::{ping::*};
use crate::commands::staff::{ban::*, kick::*, unban::*};
use crate::commands::user::avatar::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(avatar)]
struct User;

#[group]
#[commands(coinflip, dadjoke, roll, eightball, waifu, cat, dog)]
struct Fun;

#[group]
#[commands(ping)]
struct Misc;

#[group]
#[only_in("guilds")]
#[commands(kick, ban, unban)]
struct Staff;

#[group]
#[commands(shutdown)]
struct Dev;
