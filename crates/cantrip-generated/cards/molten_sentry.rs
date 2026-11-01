// Molten Sentry
// status: ready
// As this creature enters, flip a coin. If the coin comes up heads, this creature enters as a 5/2 creature with haste. If it comes up tails, this creature enters as a 2/5 creature with defender.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let flip_0 = g.flip_coin(player_0);
if { let q: &dyn Query = g; flip_0 } {
entering.copy = Some({ let q: &dyn Query = g; q.copiable_values(entering.object).sized(5, 2).with_type(CardType::Creature).with_keyword(Keyword::Haste) });
} else {
entering.copy = Some({ let q: &dyn Query = g; q.copiable_values(entering.object).sized(2, 5).with_type(CardType::Creature).with_keyword(Keyword::Defender) });
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Molten Sentry", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, enters_as_copy: Some(ability_0_enters), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
