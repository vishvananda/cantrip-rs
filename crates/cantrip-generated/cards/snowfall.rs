// Snowfall
// status: planned
// requires: engine: only a mana ability triggers on tapping for mana
// requires: mana spending purpose
// Cumulative upkeep {U} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)
// Whenever an Island is tapped for mana, its controller may add an additional {U}. If that Island is snow, its controller may add an additional {U}{U} instead. Spend this mana only to pay cumulative upkeep costs.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| q.chars(candidate).supertypes.snow) } {
let decider_0 = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let mana_players_2 = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
for player in mana_players_2 {
g.add_restricted_mana(player, Mana::U, mana_2_0, planned_value!("mana spending purpose", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: |q, asker, spending| spending.part == ManaPart::CumulativeUpkeep, spent: None })));}
g.end_sentence();
}
g.end_sentence();
} else {
let decider_3 = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_3, asker.source, Purpose::Optional)? {
let mana_players_5 = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() };
let mana_5_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_5 {
g.add_restricted_mana(player, Mana::U, mana_5_0, planned_value!("mana spending purpose", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: |q, asker, spending| spending.part == ManaPart::CumulativeUpkeep, spent: None })));}
g.end_sentence();
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("engine-only-a-mana-ability-triggers-on-tapping-for-mana" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::TappedForMana { permanent, player, produced } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), Mana::ALL.iter().filter(|&&m| produced.get(m) > 0).fold(0u32, |types, &m| types | 1 << m as u32)]; if q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Island)) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Snowfall", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::cumulative_upkeep!(cantrip_mechanics::cumulative_upkeep::Payment::Mana(ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), "Cumulative upkeep"),triggered(when_asked(planned_value!("engine: only a mana ability triggers on tapping for mana", EventMask(0) => EventMask(EventMask::TAPPED.0)), ability_1_event), ability_1, "Whenever an Island is tapped for mana, its controller may add an additional {U}. If that Island is snow, its controller may add an additional {U}{U} instead. Spend this mana only to pay cumulative upkeep costs.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
