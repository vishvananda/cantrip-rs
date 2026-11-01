// Fear of Change
// status: planned
// requires: operation Conjure
// When this creature enters or dies, exile another creature you control. If you do, conjure a random creature card with mana value X onto the battlefield, where X is 2 plus the exiled creature's mana value.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let selected_1 = if selected_1_candidates.len() as u64 >= u64::from(selected_1_count) { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Exile, selected_1_candidates, cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::Exactly(selected_1_count), &[]))? } else { Vec::new() };
let objects_0 = { let q: &dyn Query = g; selected_1.clone() }; let moved_0 = g.exile_all(&objects_0);
let paid_0 = moved_0.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let count_2: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_2: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_2.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::AtRandom(|q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) == ((2i64).saturating_add((moved_0.clone()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).sum::<i64>())))), count_2, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
hoisted_3.extend(conjured_2.clone());
hoisted_4.extend(conjured_2.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Fear of Change", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Nightmare]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters or dies, exile another creature you control. If you do, conjure a random creature card with mana value X onto the battlefield, where X is 2 plus the exiled creature's mana value.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
