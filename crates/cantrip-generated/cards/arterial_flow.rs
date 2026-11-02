// Arterial Flow
// status: ready
// Each opponent discards two cards. If you control a Vampire, each opponent loses 2 life and you gain 2 life.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_0 = Vec::new(); for player in g.state().apnap().filter(|p| players_0.contains(p)).collect::<Vec<PlayerId>>() { cards_0.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_0)?); } let discarded_0 = g.discard_together(&cards_0);
g.end_sentence();
if { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Vampire))).count() >= 1 } {
let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.gain_life(player, amount_2); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Arterial Flow", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
