// Emperor Apatzec Intli IV
// status: planned
// requires: engine: no perpetual effects
// Whenever another creature you control enters, that creature perpetually gains haste if its power is 4 or greater. If its toughness is 4 or greater, you gain 4 life. If its mana value is 4 or greater, seek a creature card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (!(Some(asker.source) == Some(id)) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| q.chars(candidate).power.is_some_and(|v| (i64::from(v)) >= (4i64))) } {
planned!("engine-no-perpetual-effects" => let objects_1: Vec<ObjectId> = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() };
let kind_1_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste]))) };
for &object in &objects_1 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_1_0); });
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| q.chars(candidate).toughness.is_some_and(|v| (i64::from(v)) >= (4i64))) } {
let amount_2 = { let q: &dyn Query = g; (4i64).max(0) as i32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.gain_life(player, amount_2); }
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| (i64::from(q.mana_value(candidate))) >= (4i64)) } {
let mut pool_3 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let count_3 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_3: Vec<ObjectId> = Vec::new(); while picked_3.len() < count_3 && !pool_3.is_empty() { let at = g.choose_at_random(pool_3.len()); picked_3.push(pool_3.remove(at)); }
let objects_4 = { let q: &dyn Query = g; picked_3.clone() }; let moved_4 = g.return_to_hand(&objects_4);
g.end_sentence();
hoisted_5.extend(moved_4.clone());
hoisted_6.extend(picked_3.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Emperor Apatzec Intli IV", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Noble]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Whenever another creature you control enters, that creature perpetually gains haste if its power is 4 or greater. If its toughness is 4 or greater, you gain 4 life. If its mana value is 4 or greater, seek a creature card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
