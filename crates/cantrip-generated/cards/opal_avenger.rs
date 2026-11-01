// Opal Avenger
// status: ready
// When you have 10 or less life, if this permanent is an enchantment, it becomes a 3/5 Soldier creature.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_state(q: &dyn Query, asker: &Asker) -> bool { (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().any(|bound_0: PlayerId| (Some(bound_0).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))) <= (10i64)) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::Indefinite;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::SetPt((3i64) as i16, (5i64) as i16) };
let kind_0_1_0 = { let q: &dyn Query = g; EffectKind::SetCreatureType(q.words(asker.source).subtype(Subtype::Soldier)) };
let kind_0_2_0 = { let q: &dyn Query = g; EffectKind::SetTypes(TypeSet::of(&[CardType::Creature])) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0); g.add_effect(source, object, kind_0_1_0, duration_0); g.add_effect(source, object, kind_0_2_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Opal Avenger", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[state_triggered(ability_0_state, ability_0, "When you have 10 or less life, if this permanent is an enchantment, it becomes a 3/5 Soldier creature.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
