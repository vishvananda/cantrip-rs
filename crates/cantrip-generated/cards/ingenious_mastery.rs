// Ingenious Mastery
// status: ready
// You may pay {2}{U} rather than pay this spell's mana cost.
// If the {2}{U} cost was paid, you draw three cards, then an opponent creates two Treasure tokens and they scry 2. If that cost wasn't paid, you draw X cards.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: token!(Treasure).name, token_words: token!(Treasure).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Treasure]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "Add mana.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; item.way == 1 } {
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mut done_0: i64 = 0; let mut drawn_0: Vec<ObjectId> = Vec::new(); for player in players_0 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_0); done_0 += g.history().drawn(player) as i64 - before; drawn_0.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
g.end_sentence();
let among_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(doer_1) = (if among_1.len() > 1 { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among_1, "Choose", 0)? } else { among_1.first().copied() }) else { return Ok(()); }; { let asker = &Asker::new(asker.source, doer_1);
let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_2 = Vec::new();
let count_2_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let def_2_0 = g.card_id(&token_3::CARD);
for player in players_2 {
tokens_2.extend(std::iter::repeat((def_2_0,player)).take(count_2_0 as usize));
} let words_2 = Words::NONE;
let created_2 = g.create_tokens_worded(&tokens_2, false, None, words_2)?;
g.end_sentence();
hoisted_4.extend(created_2.clone());
let amount_5 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_5, asker.source, amount_5)?;
g.end_sentence();
}
g.end_sentence();
hoisted_6.extend(drawn_0.clone());
hoisted_7.extend(hoisted_4.clone());
} else {
let amount_8 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as u32 }; let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_8 { g.draw(player, amount_8); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Ingenious Mastery", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 1, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ZoneKind::Hand, "You may pay {2}{U} rather than pay this spell's mana cost.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD];
