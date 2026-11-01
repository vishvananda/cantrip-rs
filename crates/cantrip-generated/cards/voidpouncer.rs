// Voidpouncer
// status: ready
// Devoid (This card has no color.)
// Kicker {2}{C} (You may pay an additional {2}{C} as you cast this spell.)
// If this creature was kicked, it enters with two +1/+1 counters and a trample counter on it and with haste.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.colors = ColorSet::NONE; }
#[allow(unused_variables, unused_mut)] fn ability_2_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); if { let q: &dyn Query = g; Target::Object(asker.source).object().is_some_and(|candidate: ObjectId| q.state().obj(candidate).kicked > 0) } {
let more = { let q: &dyn Query = g; (2i64).max(0) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
let more = { let q: &dyn Query = g; (1i64).max(0) as i16 }; entering.counters[CounterKind::Trample as usize] = entering.counters[CounterKind::Trample as usize].saturating_add(more);
static WHAT_3: EntersAs = EntersAs { abilities: &[], keywords: Keywords::of(&[Keyword::Haste]), ..EntersAs::NONE }; entering.becomes = Some(Becomes { what: &WHAT_3, card: { let q: &dyn Query = g; { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) } }, words: g.words(entering.object) });
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Voidpouncer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[characteristic_defining(Layer::Color, ability_0_apply_0, "Devoid")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_2_enters), modal: None, enchant: None, kicker: kicker(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 1], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
