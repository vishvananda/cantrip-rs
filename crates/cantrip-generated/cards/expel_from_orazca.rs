// Expel from Orazca
// status: planned
// requires: engine: no effect gives the city's blessing
// Ascend (If you control ten or more permanents, you get the city's blessing for the rest of the game.)
// Return target nonland permanent to its owner's hand. If you have the city's blessing, you may put that permanent on top of its owner's library instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (!(q.chars(id).is(CardType::Land)))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("engine-no-effect-gives-the-city-s-blessing" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.ascend(player); });
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).citys_blessing) } {
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_1 = cantrip_mechanics::effects::ask_may(g, payer_1, asker.source, Purpose::Optional)?; if paid_1 {
let objects_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_2 = g.put_into_library(&objects_2, LibraryPlace::Top);
g.end_sentence();
hoisted_3.extend(moved_2.clone());
hoisted_4.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_1 as i64) != 0)) } {
let objects_5 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_5 = g.return_to_hand(&objects_5);
g.end_sentence();
hoisted_6.extend(moved_5.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_8.extend(hoisted_3.clone());
hoisted_9.extend(hoisted_6.clone());
hoisted_10.extend(hoisted_3.clone());
hoisted_11.extend({ let q: &dyn Query = g; hoisted_3.clone() });
hoisted_12.extend({ let q: &dyn Query = g; hoisted_7.clone() });
hoisted_13.extend({ let q: &dyn Query = g; hoisted_4.clone() });
} else {
let objects_14 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_14 = g.return_to_hand(&objects_14);
g.end_sentence();
hoisted_15.extend(moved_14.clone());
hoisted_16.extend({ let q: &dyn Query = g; objects_14.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Expel from Orazca", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
