// Dust Animus
// status: planned
// requires: conditional Replacement
// Flying
// If you control five or more untapped lands, this creature enters with two +1/+1 counters and a lifelink counter on it.
// Plot {1}{W} (You may pay {1}{W} and exile this card from your hand. Cast it as a sorcery on a later turn without paying its mana cost. Plot only as a sorcery.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_stated_then_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (2i64).max(0) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
let more = { let q: &dyn Query = g; (1i64).max(0) as i16 }; entering.counters[CounterKind::Lifelink as usize] = entering.counters[CounterKind::Lifelink as usize].saturating_add(more);
 Ok(()) }
planned_item!("conditional-replacement" => enters: cantrip_mechanics::zones::when(|q: &dyn Query, asker: &Asker| -> bool { ((q.state().battlefield().iter().copied().filter(|&id| ((!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Land)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (5i64) }, ability_1_stated_then_0_enters));

pub static CARD: CardDef = CardDef { name: "Dust Animus", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::plot::plot(ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Plot {1}{W}")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
