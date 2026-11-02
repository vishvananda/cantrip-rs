// Lifestream's Blessing
// status: ready
// Draw X cards, where X is the greatest power among creatures you controlled as you cast this spell. If this spell was cast from exile, you gain twice X life.
// Foretell {4}{G} (During your turn, you may pay {2} and exile this card from your hand face down. Cast it on a later turn for its foretell cost.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let value_0: i64 = { let q: &dyn Query = g; i64::from(q.history().fact_as_cast(item.object)) };
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (value_0).clamp(0, 4294967295) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| cast_of(q, candidate).is_some_and(|cast| cast.from == ZoneKind::Exile)) } {
let amount_2 = { let q: &dyn Query = g; ((2i64).saturating_mul(value_0)).clamp(0, 2147483647) as i32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.gain_life(player, amount_2); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn kept_as_cast(q: &dyn Query, asker: &Asker) -> u32 { ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter_map(|o: ObjectId| q.chars(o).power).map(i64::from).max().unwrap_or(0)).max(0) as u32 }
fn cast_of<'a>(q: &'a dyn Query, id: ObjectId) -> Option<&'a Cast> { let records = q.history().all(); let spell = records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == id => Some(old), _ => None }).unwrap_or(id); records.iter().rev().find_map(|r| match &r.event { Event::Cast(cast) if cast.spell == spell => Some(cast), _ => None }) }
#[allow(dead_code)] fn cast_at<'a>(q: &'a dyn Query, event: &Event, id: ObjectId) -> Option<&'a Cast> { match event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if *new == id => cast_of(q, *old), _ => cast_of(q, id) } }

pub static CARD: CardDef = CardDef { name: "Lifestream's Blessing", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[cantrip_mechanics::foretell::ABILITY], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_fact: Some(kept_as_cast), foretell: Some(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
