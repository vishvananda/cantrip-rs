// Invasion of Pyrulea // Gargantuan Slabhorn
// status: ready
// Trample, ward {2}
// Other transformed permanents you control have trample and ward {2}.
// (As a Siege enters, choose an opponent to protect it. You and others can attack it. When it's defeated, exile it, then cast it transformed.)
// When this Siege enters, scry 3, then reveal the top card of your library. If it's a land or double-faced card, draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_0, asker.source, amount_0)?;
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
if { let q: &dyn Query = g; (((objects_1.clone()).into_iter().filter(|&id| (q.chars(id).is(CardType::Land) || q.state().obj(id).represented_double_faced(q.card(q.state().obj(id).card).back.is_some()))).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) } {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Invasion of Pyrulea", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Battle]), subtypes: SubtypeSet::of(&[Subtype::Siege]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: Some(4), keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this Siege enters, scry 3, then reveal the top card of your library. If it's a land or double-faced card, draw a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source) == Some(affected)) && (q.state().obj(affected).transformed && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Trample);
let card = q.cards().current(q.state().obj(asker.source)); let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed(q.cards().current(q.state().obj(asker.source)), 0)); } }

pub static CARD: CardDef = CardDef { name: "Gargantuan Slabhorn", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(18)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward::<2>("Ward {2}")], statics: &[static_ability(Layer::Ability, ability_2_affects, ability_2_apply_0, "Other transformed permanents you control have trample and ward {2}.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[cantrip_mechanics::triggers::ward::<2>("Ward {2}")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
