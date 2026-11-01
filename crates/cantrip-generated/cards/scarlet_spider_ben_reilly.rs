// Scarlet Spider, Ben Reilly
// status: planned
// requires: conditional Replacement
// requires: earlier objects Webslung
// Web-slinging {R}{G} (You may cast this spell for {R}{G} if you also return a tapped creature you control to its owner's hand.)
// Trample
// Sensational Save — If Scarlet Spider was cast using web-slinging, he enters with X +1/+1 counters on him, where X is the mana value of the returned creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_2_stated_then_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; ((planned_value!("earlier objects Webslung", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Webslung))).into_iter().next().map_or(0i64, |id: ObjectId| i64::from(q.mana_value(id)))).clamp(0, 32767) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
 Ok(()) }
planned_item!("conditional-replacement" => enters: cantrip_mechanics::zones::when(|q: &dyn Query, asker: &Asker| -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| { let o = q.state().obj(candidate); o.cast_way > 0 && q.card(o.card).ways.get(o.cast_way as usize - 1).is_some_and(|w| w.kind == WayKind::WebSlinging) }) }, ability_2_stated_then_0_enters));

pub static CARD: CardDef = CardDef { name: "Scarlet Spider, Ben Reilly", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spider,Subtype::Human,Subtype::Hero]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::web_slinging::way(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Web-slinging {R}{G}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
