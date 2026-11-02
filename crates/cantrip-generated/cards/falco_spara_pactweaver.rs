// Falco Spara, Pactweaver
// status: planned
// requires: play permission from the top of a library on other terms
// Flying, trample
// Falco Spara enters with a shield counter on it.
// You may look at the top card of your library any time.
// You may cast spells from the top of your library by removing a counter from a creature you control in addition to paying their other costs.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_2_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (1i64).max(0) as i16 }; entering.counters[CounterKind::Shield as usize] = entering.counters[CounterKind::Shield as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_4_0_cost_1(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) }
planned_item!("play-permission-from-the-top-of-a-library-on-other-terms" => #[allow(unused_variables)] fn ability_4_0_cards_0(q: &dyn Query, asker: &Asker, card: ObjectId) -> bool { true });
planned_item!("play-permission-from-the-top-of-a-library-on-other-terms" => play_rules: PlayRule::LibraryTopSpellsWith(ability_4_0_cards_0, LibraryTerms { additional: Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(remove_counters_among(1, ability_4_0_cost_1)), flash: false }));

pub static CARD: CardDef = CardDef { name: "Falco Spara, Pactweaver", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird,Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_2_enters), modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
