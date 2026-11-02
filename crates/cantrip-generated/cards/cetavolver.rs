// Cetavolver
// status: ready
// Kicker {1}{R} and/or {G} (You may pay an additional {1}{R} and/or {G} as you cast this spell.)
// If this creature was kicked with its {1}{R} kicker, it enters with two +1/+1 counters on it and with first strike.
// If this creature was kicked with its {G} kicker, it enters with a +1/+1 counter on it and with trample.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_2_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); if { let q: &dyn Query = g; Target::Object(asker.source).object().is_some_and(|candidate: ObjectId| q.state().obj(candidate).kicked > 0) } {
let more = { let q: &dyn Query = g; (2i64).max(0) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
static WHAT_2: EntersAs = EntersAs { abilities: &[], keywords: Keywords::of(&[Keyword::FirstStrike]), ..EntersAs::NONE }; entering.becomes = Some(Becomes { what: &WHAT_2, card: { let q: &dyn Query = g; { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) } }, words: g.words(entering.object) });
}
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_3_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); if { let q: &dyn Query = g; Target::Object(asker.source).object().is_some_and(|candidate: ObjectId| q.state().obj(candidate).paid > 0) } {
let more = { let q: &dyn Query = g; (1i64).max(0) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
static WHAT_5: EntersAs = EntersAs { abilities: &[], keywords: Keywords::of(&[Keyword::Trample]), ..EntersAs::NONE }; static WHAT_5_TOO: EntersAs = EntersAs { abilities: &[], keywords: Keywords::of(&[Keyword::FirstStrike, Keyword::Trample]), ..EntersAs::NONE }; entering.becomes = Some(Becomes { what: if entering.becomes.is_some() { &WHAT_5_TOO } else { &WHAT_5 }, card: { let q: &dyn Query = g; { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) } }, words: g.words(entering.object) });
}
 Ok(()) }
fn entering_all(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { ability_2_enters(g, entering)?; ability_3_enters(g, entering)?;  Ok(()) }

pub static CARD: CardDef = CardDef { name: "Cetavolver", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Volver]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(entering_all), modal: None, enchant: None, kicker: kicker(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), labels: &["Cast without kicking", "Kicker {G}"], optional_cost: Some(OptionalCost::new(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
