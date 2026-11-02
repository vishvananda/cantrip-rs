// Canker Abomination
// status: ready
// As this creature enters, choose an opponent. This creature enters with a -1/-1 counter on it for each creature that player controls.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = entering.controller; let candidates_0 = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_0) = cantrip_mechanics::choose::player(g, player_0, entering.object, candidates_0, "Choose", 0)? else { return Ok(()) };
entering.chosen_player = Some(chosen_0);
let more = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(chosen_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).min(i16::MAX.into()) as i16 }; entering.counters[CounterKind::MinusOne as usize] = entering.counters[CounterKind::MinusOne as usize].saturating_add(more);
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Canker Abomination", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::B, Mana::G)),Some((Mana::B, Mana::G)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Treefolk,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
