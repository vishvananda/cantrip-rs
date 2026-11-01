// Dralnu's Pet
// status: planned
// requires: Kicker cost other than mana
// requires: earlier objects Discarded
// Kicker—{2}{B}, Discard a creature card. (You may pay {2}{B} and discard a creature card in addition to any other costs as you cast this spell.)
// If this creature was kicked, it enters with flying and with X +1/+1 counters on it, where X is the discarded card's mana value.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_way_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); if { let q: &dyn Query = g; Target::Object(asker.source).object().is_some_and(|candidate: ObjectId| q.state().obj(candidate).kicked > 0) } {
let more = { let q: &dyn Query = g; ((planned_value!("earlier objects Discarded", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Discarded))).into_iter().next().map_or(0i64, |id: ObjectId| i64::from(q.mana_value(id)))).clamp(0, 32767) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
static WHAT_3: EntersAs = EntersAs { abilities: &[], keywords: Keywords::of(&[Keyword::Flying]), ..EntersAs::NONE }; entering.becomes = Some(Becomes { what: &WHAT_3, card: { let q: &dyn Query = g; { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) } }, words: g.words(entering.object) });
}
 Ok(()) }
planned_item!("kicker-cost-other-than-mana" => ways: cantrip_mechanics::kicker::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(discard(1, ability_0_way_0)), "Kicker"));

pub static CARD: CardDef = CardDef { name: "Dralnu's Pet", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shapeshifter]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
