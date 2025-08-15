use anchor_lang::{prelude::*, solana_program::sysvar};
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::ID_CONST;

use crate::{
    error::ErrorCode,
    state::{Market, UserWager},
    COMP_DEF_OFFSET_RESOLVE_MARKET, ID,
};
