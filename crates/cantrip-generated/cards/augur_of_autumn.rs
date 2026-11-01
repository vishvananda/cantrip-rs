// Augur of Autumn
// status: ready
// You may look at the top card of your library any time.
// You may play lands from the top of your library.
// Coven — As long as you control three or more creatures with different powers, you may cast creature spells from the top of your library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { q.chars(spell).is(CardType::Creature) }
#[allow(unused_variables)] fn ability_2_0_0_while(q: &dyn Query, asker: &Asker) -> bool { ({ let mut seen = Vec::new(); for id in q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() { let value = q.chars(id).power; if !seen.contains(&value) { seen.push(value); } } seen.len() as i64 }) >= (3i64) }

pub static CARD: CardDef = CardDef { name: "Augur of Autumn", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Druid]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopLands,PlayRule::While(ability_2_0_0_while, &[PlayRule::LibraryTopSpells(ability_2_0_0_rule, None)])], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
