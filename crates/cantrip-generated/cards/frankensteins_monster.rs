// Frankenstein's Monster
// status: planned
// requires: event change as it enters Entry CountersOfChoice
// requires: power/toughness counter other than ±1/±1
// As this creature enters, exile X creature cards from your graveyard. If you can't, put this creature into its owner's graveyard instead of onto the battlefield. For each creature card exiled this way, this creature enters with a +2/+0, +1/+1, or +0/+2 counter on it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let selected_1_player = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; (i64::from(entering.x)) as u32 }; let selected_1 = if selected_1_candidates.len() as u64 >= u64::from(selected_1_count) { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Exile, selected_1_candidates, cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::Exactly(selected_1_count), &[]))? } else { Vec::new() };
let objects_0 = { let q: &dyn Query = g; selected_1.clone() }; let moved_0 = g.exile_all(&objects_0);
let paid_0 = moved_0.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
planned!("event-change-as-it-enters-entry-counters-of-choice" => let many = { let q: &dyn Query = g; (((moved_0.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).len() as i64) as u32 }; for _ in 0..many { cantrip_mechanics::choose::counter_as_enters(g, entering, &[planned_value!("power/toughness counter other than ±1/±1", CounterKind::PlusOne => CounterKind::Plus2Plus0), CounterKind::PlusOne, planned_value!("power/toughness counter other than ±1/±1", CounterKind::PlusOne => CounterKind::Plus0Plus2)])?; });
} else {
entering.instead = Some(Zone::graveyard(g.obj(entering.object).owner));
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Frankenstein's Monster", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
