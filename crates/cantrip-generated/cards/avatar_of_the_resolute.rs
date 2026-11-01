// Avatar of the Resolute
// status: ready
// Reach, trample
// This creature enters with a +1/+1 counter on it for each other creature you control with a +1/+1 counter on it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_2_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (!(Target::Object(asker.source).object() == Some(id)) && q.chars(id).is(CardType::Creature) && Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)) == Some(q.controller(id)) && (i64::from(q.state().obj(id).counters(CounterKind::PlusOne))) > (0i64))).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 32767) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Avatar of the Resolute", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Reach,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_2_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
