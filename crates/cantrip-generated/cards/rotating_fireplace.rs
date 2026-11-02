// Rotating Fireplace
// status: ready
// This artifact enters tapped with a time counter on it.
// {T}: Add an amount of {C} equal to the number of time counters on this artifact.
// {4}, {T}: Time travel. Activate only as a sorcery. (For each suspended card you own and each permanent you control with a time counter on it, you may add or remove a time counter.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
let more = { let q: &dyn Query = g; (1i64).max(0) as i16 }; entering.counters[CounterKind::Time as usize] = entering.counters[CounterKind::Time as usize].saturating_add(more);
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; ((Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Time))).sum::<i64>()) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::C, mana_2_0);}
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; time_travel_for(g, player, asker.source)?; }
g.end_sentence();
 Ok(()) }
fn time_travel_for(g: &mut dyn Game, player: PlayerId, source: ObjectId) -> Result<(), Suspend> { let mut each: Vec<ObjectId> = g.state().zone(Zone::exile(player)).iter().copied().filter(|&id| g.obj(id).counters(CounterKind::Time) > 0 && g.card(g.obj(id).card).suspend.is_some()).collect(); each.extend(g.permanents_of(player).into_iter().filter(|&id| g.obj(id).counters(CounterKind::Time) > 0)); let any = cantrip_mechanics::select::cards(cantrip_mechanics::select::ANY_NUMBER); let adding = cantrip_mechanics::choose::select(g, player, source, Purpose::PutCounters(CounterKind::Time, 1), each.clone(), any.clone())?; let rest: Vec<ObjectId> = each.into_iter().filter(|id| !adding.contains(id)).collect(); let removing = cantrip_mechanics::choose::select(g, player, source, Purpose::Choose, rest, any)?; let more: Vec<(ObjectId, CounterKind, i16)> = adding.iter().map(|&id| (id, CounterKind::Time, 1)).collect(); g.add_counters_together(&more); for id in removing { g.remove_counters(id, CounterKind::Time, 1); } Ok(()) }

pub static CARD: CardDef = CardDef { name: "Rotating Fireplace", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_1_mana), "{T}: Add an amount of {C} equal to the number of time counters on this artifact."),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_2, "{4}, {T}: Time travel. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
