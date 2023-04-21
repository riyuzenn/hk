
/// Unlock all skins, emotes visually for Stumbled Guys.

use crate::{hook::get_method_ptr, offset::HAS_EMOTE};
use retour::static_detour;
use std::mem::transmute;


static_detour! {
    pub static HasEmote: fn(String) -> bool;
}

type FnHasEmote = fn(String) -> bool;

fn hk_has_emote(_id: String) -> bool { true }

pub unsafe fn initialize_skin_hook() {

    let emote_mptr = get_method_ptr(HAS_EMOTE).unwrap();
    let has_emote: FnHasEmote = transmute(emote_mptr);

    HasEmote
        .initialize(has_emote, hk_has_emote)
        .unwrap();
}

