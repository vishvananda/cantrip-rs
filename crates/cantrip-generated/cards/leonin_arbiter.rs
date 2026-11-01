// Leonin Arbiter
// status: planned
// requires: ability PayToIgnore
// Players can't search libraries. Any player may pay {2} for that player to ignore this effect until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("ability-pay-to-ignore" => pay_to_ignore: PayToIgnore { subjects: Subjects::Players(|q: &dyn Query, asker: &Asker, player: PlayerId| true), rules: &[PlayerEffect::CantSearchLibraries], payer: None, cost: Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), until: Duration::EndOfTurn });

pub static CARD: CardDef = CardDef { name: "Leonin Arbiter", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
