// Akroma's Will
// status: planned
// requires: engine: no commander
// Choose one. If you control a commander as you cast this spell, you may choose both instead.
// • Creatures you control gain flying, vigilance, and double strike until end of turn.
// • Creatures you control gain lifelink, indestructible, and protection from each color until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Flying])) };
let kind_0_1_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Vigilance])) };
let kind_0_2_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::DoubleStrike])) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0); g.add_effect(source, object, kind_0_1_0, duration_0); g.add_effect(source, object, kind_0_2_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Lifelink])) };
let kind_1_1_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Indestructible])) };
let kind_1_2_0 = { let q: &dyn Query = g; EffectKind::Protection(ColorSet::NONE.with(Color::White)) };
let kind_1_3_1 = { let q: &dyn Query = g; EffectKind::Protection(ColorSet::NONE.with(Color::Blue)) };
let kind_1_4_2 = { let q: &dyn Query = g; EffectKind::Protection(ColorSet::NONE.with(Color::Black)) };
let kind_1_5_3 = { let q: &dyn Query = g; EffectKind::Protection(ColorSet::NONE.with(Color::Red)) };
let kind_1_6_4 = { let q: &dyn Query = g; EffectKind::Protection(ColorSet::NONE.with(Color::Green)) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1); g.add_effect(source, object, kind_1_1_0, duration_1); g.add_effect(source, object, kind_1_2_0, duration_1); g.add_effect(source, object, kind_1_3_1, duration_1); g.add_effect(source, object, kind_1_4_2, duration_1); g.add_effect(source, object, kind_1_5_3, duration_1); g.add_effect(source, object, kind_1_6_4, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_0_more_modes(q: &dyn Query, asker: &Asker) -> bool { q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && planned_value!("engine: no commander", false => q.is_commander(id))).count() >= 1 }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Creatures you control gain flying, vigilance, and double strike until end of turn.").targets(&[]),mode(ability_0_mode_1, "Creatures you control gain lifelink, indestructible, and protection from each color until end of turn.").targets(&[])]).choose(1, 1).up_to_if(ability_0_more_modes, 2);

pub static CARD: CardDef = CardDef { name: "Akroma's Will", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
