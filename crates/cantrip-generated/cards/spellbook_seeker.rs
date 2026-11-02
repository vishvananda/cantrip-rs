// Spellbook Seeker // Careful Study
// status: ready
// Flying
// This creature enters prepared. (While it's prepared, you may cast a copy of its spell. Doing so unprepares it.)
// Draw two cards, then discard two cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.prepared = true;
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Spellbook Seeker", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, prepare: Some(&prepared::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&prepared::CARD];
pub mod prepared {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_1 = Vec::new(); for player in players_1 { cards_1.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_1)?); } let discarded_1 = g.discard_together(&cards_1);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Careful Study", token_words: &[], oracle: false, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
