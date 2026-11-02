// Mutational Advantage
// status: ready
// Permanents you control with counters on them gain hexproof and indestructible until end of turn. Prevent all damage that would be dealt to those permanents this turn. Proliferate. (Choose any number of permanents and/or players, then give each another counter of each kind already there.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_2_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && true { return true; } }
 false }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && Some(asker.controller) == Some(q.controller(id)) && (CounterKind::ALL.iter().map(|k| i64::from(q.state().obj(id).counters(*k))).sum::<i64>()) > (0i64))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Hexproof])) };
let kind_0_1_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Indestructible])) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0); g.add_effect(source, object, kind_0_1_0, duration_0);  } }); }
g.end_sentence();
for each in { let q: &dyn Query = g; objects_0.clone() } {  let to = { let q: &dyn Query = g; (vec![Target::Object(each)]).first().copied() }; let about = { let q: &dyn Query = g; Some(each) }; if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times = 1u32; for _ in 0..times { g.proliferate(player, asker.source)?; }; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Mutational Advantage", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Prevent all damage that would be dealt to those permanents this turn. Proliferate.", damage: "damage", of_it: None, applies: made_2_applies, then: None, text: "Prevent all damage that would be dealt to those permanents this turn. Proliferate." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
