// The Curse of Fenric
// status: planned
// requires: engine: no effect changes supertypes
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — For each player, destroy up to one target creature that player controls. For each creature destroyed this way, its controller creates a 3/3 green Mutant creature token with deathtouch.
// II — Target nontoken creature becomes a 6/6 legendary Horror creature named Fenric and loses all abilities.
// III — Target Mutant fights another target creature named Fenric.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::controlled_by_slot_opponent(q, id)) && (matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature)))) }
#[allow(unused_variables)] fn ability_0_0_own_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && q.controller(id) == asker.controller) && (matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature)))) }
static ABILITY_0_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_0_target_0).between(0, TargetSpec::EACH_OPPONENT),target(ability_0_0_own_target_0).between(0, 1)];
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Mutant).name, token_words: token!(Mutant).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Mutant]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw: Vec<Target> = [announced(g, item, 0, false), announced(g, item, 1, false)].concat(); let t_0: Vec<Target> = [announced(g, item, 0, true), announced(g, item, 1, true)].concat();
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
for bound_0 in { let q: &dyn Query = g; objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() } {
let players_1 = { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let created_1 = g.create_tokens_worded(&tokens_1, false, None, words_1)?;
g.end_sentence();
hoisted_3.extend(created_1.clone());
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.state().obj(id).token) && q.chars(id).is(CardType::Creature)))) }
static ABILITY_0_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_1_target_0)];
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_4 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::Indefinite;
let kind_4_0_0 = { let q: &dyn Query = g; EffectKind::SetPt((6i64) as i16, (6i64) as i16) };
let kind_4_1_0 = { let q: &dyn Query = g; planned_value!("engine: no effect changes supertypes", EffectKind::Pump(0, 0) => EffectKind::AddSupertype(Supertype::Legendary)) };
let kind_4_2_0 = { let q: &dyn Query = g; EffectKind::SetCreatureType(q.words(asker.source).subtype(Subtype::Horror)) };
let kind_4_3_0 = { let q: &dyn Query = g; EffectKind::SetTypes(TypeSet::of(&[CardType::Creature])) };
let kind_4_4_0 = { let q: &dyn Query = g; EffectKind::SetName("Fenric") };
let kind_4_5_0 = { let q: &dyn Query = g; EffectKind::LoseAllAbilities };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4); g.add_effect(source, object, kind_4_1_0, duration_4); g.add_effect(source, object, kind_4_2_0, duration_4); g.add_effect(source, object, kind_4_3_0, duration_4); g.add_effect(source, object, kind_4_4_0, duration_4); g.add_effect(source, object, kind_4_5_0, duration_4);  } }); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Mutant)))) }
#[allow(unused_variables)] fn ability_0_2_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chosen_targets().get(0).copied().and_then(|t: Target| t.object()) == Some(id)) && (q.chars(id).is(CardType::Creature) && q.chars(id).name == "Fenric")))) }
static ABILITY_0_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_2_target_0),target(ability_0_2_target_1)];
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
let first_5 = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied(); let second_5 = ({ let q: &dyn Query = g; (t_1_raw.get(0).copied().filter(|t| t_1.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied();
if let (Some(first), Some(second)) = (first_5, second_5) { g.fight(first, second); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Curse of Fenric", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — For each player, destroy up to one target creature that player controls. For each creature destroyed this way, its controller creates a 3/3 green Mutant creature token with deathtouch.").targets(ABILITY_0_0_TARGETS),chapter(2, ability_0_1, "II — Target nontoken creature becomes a 6/6 legendary Horror creature named Fenric and loses all abilities.").targets(ABILITY_0_1_TARGETS),chapter(3, ability_0_2, "III — Target Mutant fights another target creature named Fenric.").targets(ABILITY_0_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];
