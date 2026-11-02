// Tin Street Gossip
// status: planned
// requires: mana spending purpose
// Vigilance
// {T}: Add {R}{G}. Spend this mana only to cast face-down spells or to turn creatures face up.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("mana-spending-purpose" => #[allow(dead_code, unused_variables)] fn mana_use_0(q: &dyn Query, asker: &Asker, spending: &Spending) -> bool { (spending.purpose == SpendingFor::Casting && spending.source.is_some_and(|spell| q.state().obj(spell).face_down)) || (spending.part == ManaPart::TurnFaceUp) });

pub static CARD: CardDef = CardDef { name: "Tin Street Gossip", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Lizard,Subtype::Advisor]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 1, 1, 0]), ..add_each(&[]) }.only_for(planned_value!("mana spending purpose", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: mana_use_0, spent: None }))), "{T}: Add {R}{G}. Spend this mana only to cast face-down spells or to turn creatures face up.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
