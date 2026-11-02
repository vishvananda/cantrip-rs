// Sword of the Ages
// status: planned
// requires: sacrificing any number of permanents to activate an ability
// This artifact enters tapped.
// {T}, Sacrifice this artifact and any number of creatures you control: This artifact deals X damage to any target, where X is the total power of the creatures sacrificed this way, then exile this artifact and those creature cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let amount_1 = { let q: &dyn Query = g; (({ let mut all: Vec<ObjectId> = Vec::new(); for id in vec![item.object] { if !all.contains(&id) { all.push(id); } } for id in q.cost_objects(item).to_vec() { if !all.contains(&id) { all.push(id); } } all }).into_iter().filter_map(|o: ObjectId| q.chars(o).power).map(i64::from).sum::<i64>()).clamp(0, 65535) as u16 }; let recipients_1 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_1 { let batch: Vec<(ObjectId, Target, u16)> = recipients_1.iter().map(|t| (source, *t, amount_1)).collect(); g.damage_together(&batch); }
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in (vec![item.object]).into_iter().filter_map(|o: ObjectId| q.state().obj(o).became).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in (q.cost_objects(item).to_vec()).into_iter().filter_map(|o: ObjectId| q.state().obj(o).became).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_3(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Sword of the Ages", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }).and(planned_value!("sacrificing any number of permanents to activate an ability", Cost::NONE => Cost { variable: Some(VariableCost::SacrificeAnyNumber { filter: ability_1_3, less_each: 0 }), ..Cost::NONE })), ability_1, "{T}, Sacrifice this artifact and any number of creatures you control: This artifact deals X damage to any target, where X is the total power of the creatures sacrificed this way, then exile this artifact and those creature cards.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
