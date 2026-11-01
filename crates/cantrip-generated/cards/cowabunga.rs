// Cowabunga!
// status: ready
// Look at the top four cards of your library. You may reveal a Mutant, Ninja, Turtle, or land card from among them and put it into your hand. Put the rest on the bottom of your library in a random order.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let depth_0 = { let q: &dyn Query = g; (4i64).max(0) as usize }; let word_0_0 = { let q: &dyn Query = g; q.words(asker.source).subtype(Subtype::Mutant) }; let word_0_1 = { let q: &dyn Query = g; q.words(asker.source).subtype(Subtype::Ninja) }; let word_0_2 = { let q: &dyn Query = g; q.words(asker.source).subtype(Subtype::Turtle) }; let taken_0 = cantrip_mechanics::library::look_at_top(g, player_0, asker.source, depth_0, (0, 1), true, |c| (c.has_subtype(word_0_0) || c.has_subtype(word_0_1) || c.has_subtype(word_0_2) || c.is(CardType::Land)), cantrip_mechanics::library::Rest::BottomRandom)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Cowabunga!", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
