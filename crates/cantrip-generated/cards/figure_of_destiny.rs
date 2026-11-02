// Figure of Destiny
// status: ready
// {R/W}: This creature becomes a Kithkin Spirit with base power and toughness 2/2.
// {R/W}{R/W}{R/W}: If this creature is a Spirit, it becomes a Kithkin Spirit Warrior with base power and toughness 4/4.
// {R/W}{R/W}{R/W}{R/W}{R/W}{R/W}: If this creature is a Warrior, it becomes a Kithkin Spirit Warrior Avatar with base power and toughness 8/8, flying, and first strike.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::Indefinite;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::SetCreatureType(q.words(asker.source).subtype(Subtype::Kithkin)) };
let kind_0_1_1 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Spirit)) };
let kind_0_2_0 = { let q: &dyn Query = g; EffectKind::SetPt((2i64) as i16, (2i64) as i16) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0); g.add_effect(source, object, kind_0_1_1, duration_0); g.add_effect(source, object, kind_0_2_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| q.chars(candidate).has_subtype(q.words(asker.source).subtype(Subtype::Spirit))) } {
let objects_1 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::Indefinite;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::SetCreatureType(q.words(asker.source).subtype(Subtype::Kithkin)) };
let kind_1_1_1 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Spirit)) };
let kind_1_2_2 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Warrior)) };
let kind_1_3_0 = { let q: &dyn Query = g; EffectKind::SetPt((4i64) as i16, (4i64) as i16) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1); g.add_effect(source, object, kind_1_1_1, duration_1); g.add_effect(source, object, kind_1_2_2, duration_1); g.add_effect(source, object, kind_1_3_0, duration_1);  } }); }
g.end_sentence();
hoisted_2.extend(objects_1.clone());
}
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| q.chars(candidate).has_subtype(q.words(asker.source).subtype(Subtype::Warrior))) } {
let objects_3 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::Indefinite;
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::SetCreatureType(q.words(asker.source).subtype(Subtype::Kithkin)) };
let kind_3_1_1 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Spirit)) };
let kind_3_2_2 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Warrior)) };
let kind_3_3_3 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Avatar)) };
let kind_3_4_0 = { let q: &dyn Query = g; EffectKind::SetPt((8i64) as i16, (8i64) as i16) };
let kind_3_5_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Flying])) };
let kind_3_6_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::FirstStrike])) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3); g.add_effect(source, object, kind_3_1_1, duration_3); g.add_effect(source, object, kind_3_2_2, duration_3); g.add_effect(source, object, kind_3_3_3, duration_3); g.add_effect(source, object, kind_3_4_0, duration_3); g.add_effect(source, object, kind_3_5_0, duration_3); g.add_effect(source, object, kind_3_6_0, duration_3);  } }); }
g.end_sentence();
hoisted_4.extend(objects_3.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Figure of Destiny", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::R, Mana::W)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Kithkin]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::R, Mana::W)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{R/W}: This creature becomes a Kithkin Spirit with base power and toughness 2/2.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::R, Mana::W)),Some((Mana::R, Mana::W)),Some((Mana::R, Mana::W)),None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{R/W}{R/W}{R/W}: If this creature is a Spirit, it becomes a Kithkin Spirit Warrior with base power and toughness 4/4.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::R, Mana::W)),Some((Mana::R, Mana::W)),Some((Mana::R, Mana::W)),Some((Mana::R, Mana::W)),Some((Mana::R, Mana::W)),Some((Mana::R, Mana::W))], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}: If this creature is a Warrior, it becomes a Kithkin Spirit Warrior Avatar with base power and toughness 8/8, flying, and first strike.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
