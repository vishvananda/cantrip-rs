// Wibbly-wobbly, Timey-wimey
// status: ready
// Time travel. (For each suspended card you own and each permanent you control with a time counter on it, you may add or remove a time counter.)
// Draw a card.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; time_travel_for(g, player, asker.source)?; }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }
fn time_travel_for(g: &mut dyn Game, player: PlayerId, source: ObjectId) -> Result<(), Suspend> { let mut each: Vec<ObjectId> = g.state().zone(Zone::exile(player)).iter().copied().filter(|&id| g.obj(id).counters(CounterKind::Time) > 0 && g.card(g.obj(id).card).suspend.is_some()).collect(); each.extend(g.permanents_of(player).into_iter().filter(|&id| g.obj(id).counters(CounterKind::Time) > 0)); let any = cantrip_mechanics::select::cards(cantrip_mechanics::select::ANY_NUMBER); let adding = cantrip_mechanics::choose::select(g, player, source, Purpose::PutCounters(CounterKind::Time, 1), each.clone(), any.clone())?; let rest: Vec<ObjectId> = each.into_iter().filter(|id| !adding.contains(id)).collect(); let removing = cantrip_mechanics::choose::select(g, player, source, Purpose::Choose, rest, any)?; let more: Vec<(ObjectId, CounterKind, i16)> = adding.iter().map(|&id| (id, CounterKind::Time, 1)).collect(); g.add_counters_together(&more); for id in removing { g.remove_counters(id, CounterKind::Time, 1); } Ok(()) }

pub static CARD: CardDef = CardDef { name: "Wibbly-wobbly, Timey-wimey", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
