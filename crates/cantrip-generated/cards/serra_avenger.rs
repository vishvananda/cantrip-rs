// Serra Avenger
// status: ready
// You can't cast Serra Avenger during your first, second, or third turns of the game.
// Flying, vigilance
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { !(((Some(asker.controller).map_or(0i64, |p: PlayerId| q.history().all().iter().filter(|r| matches!(r.event, Event::TurnBegan { player, .. } if player == p)).count() as i64)) == (1i64) || (Some(asker.controller).map_or(0i64, |p: PlayerId| q.history().all().iter().filter(|r| matches!(r.event, Event::TurnBegan { player, .. } if player == p)).count() as i64)) == (2i64) || (Some(asker.controller).map_or(0i64, |p: PlayerId| q.history().all().iter().filter(|r| matches!(r.event, Event::TurnBegan { player, .. } if player == p)).count() as i64)) == (3i64))) }

pub static CARD: CardDef = CardDef { name: "Serra Avenger", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_only_if: Some(ability_0_if), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
