// Teferi's Contingency
// status: planned
// requires: engine: no perpetual effects
// Counter target spell. Each card in its controller's graveyard, hand, and library with the same name as that spell perpetually gain "This spell costs {2} more to cast."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && (true)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
static PERPETUAL_3: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))), ..CardDef::BLANK };
static PERPETUAL_6: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))), ..CardDef::BLANK };
static PERPETUAL_9: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))), ..CardDef::BLANK };
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; cantrip_api::instruction(g, |g| { for spell in objects_0 { g.counter(spell); } });
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; ((t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && t_0_raw.get(0).copied().and_then(|t: Target| t.object()).is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect::<Vec<ObjectId>>() };
let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_3) };
for &object in &objects_2 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); });
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_5: Vec<ObjectId> = { let q: &dyn Query = g; ((t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && t_0_raw.get(0).copied().and_then(|t: Target| t.object()).is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect::<Vec<ObjectId>>() };
let kind_5_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_6) };
for &object in &objects_5 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_5_0); });
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_8: Vec<ObjectId> = { let q: &dyn Query = g; ((t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && t_0_raw.get(0).copied().and_then(|t: Target| t.object()).is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect::<Vec<ObjectId>>() };
let kind_8_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_9) };
for &object in &objects_8 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_8_0); });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Teferi's Contingency", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
