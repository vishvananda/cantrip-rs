// Warden of the First Tree
// status: ready
// {1}{W/B}: This creature becomes a Human Warrior with base power and toughness 3/3.
// {2}{W/B}{W/B}: If this creature is a Warrior, it becomes a Human Spirit Warrior with trample and lifelink.
// {3}{W/B}{W/B}{W/B}: If this creature is a Spirit, put five +1/+1 counters on it.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::Indefinite;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::SetCreatureType(q.words(asker.source).subtype(Subtype::Human)) };
let kind_0_1_1 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Warrior)) };
let kind_0_2_0 = { let q: &dyn Query = g; EffectKind::SetPt((3i64) as i16, (3i64) as i16) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0); g.add_effect(source, object, kind_0_1_1, duration_0); g.add_effect(source, object, kind_0_2_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| q.chars(candidate).has_subtype(q.words(asker.source).subtype(Subtype::Warrior))) } {
let objects_1 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::Indefinite;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::SetCreatureType(q.words(asker.source).subtype(Subtype::Human)) };
let kind_1_1_1 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Spirit)) };
let kind_1_2_2 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Warrior)) };
let kind_1_3_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Trample])) };
let kind_1_4_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Lifelink])) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1); g.add_effect(source, object, kind_1_1_1, duration_1); g.add_effect(source, object, kind_1_2_2, duration_1); g.add_effect(source, object, kind_1_3_0, duration_1); g.add_effect(source, object, kind_1_4_0, duration_1);  } }); }
g.end_sentence();
hoisted_2.extend(objects_1.clone());
}
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| q.chars(candidate).has_subtype(q.words(asker.source).subtype(Subtype::Spirit))) } {
let mut counters_3: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (5i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_3.push((object, CounterKind::PlusOne, amount)); } }
let had_3: Vec<_> = counters_3.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_3); let applied_3: Vec<ObjectId> = counters_3.iter().zip(had_3).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_4.extend(applied_3.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Warden of the First Tree", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::W, Mana::B)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{1}{W/B}: This creature becomes a Human Warrior with base power and toughness 3/3.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::W, Mana::B)),Some((Mana::W, Mana::B)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{2}{W/B}{W/B}: If this creature is a Warrior, it becomes a Human Spirit Warrior with trample and lifelink.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::W, Mana::B)),Some((Mana::W, Mana::B)),Some((Mana::W, Mana::B)),None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{3}{W/B}{W/B}{W/B}: If this creature is a Spirit, put five +1/+1 counters on it.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
