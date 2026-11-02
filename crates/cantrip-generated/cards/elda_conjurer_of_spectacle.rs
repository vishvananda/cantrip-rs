// Elda, Conjurer of Spectacle
// status: planned
// requires: object filter NthCast
// The first nonlegendary creature spell you cast each turn has demonstrate. (When you cast that spell, you may copy it. If you do, choose an opponent to also copy it. Each copy becomes a token.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("object-filter-nth-cast" => statics: cantrip_mechanics::demonstrate::spells_have(|q: &dyn Query, asker: &Asker, id: ObjectId| -> bool { ((!(q.chars(id).supertypes.legendary) && q.chars(id).is(CardType::Creature)) && q.history().this_turn().iter().filter_map(|r| match r.event { Event::Cast(cast) => Some(cast), _ => None }).take_while(|cast| cast.spell != id).filter(|cast| Some(asker.controller) == Some(cast.player) && (!(cast.chars(q).supertypes.legendary) && cast.chars(q).is(CardType::Creature))).count() as i64 + 1 == 1i64) }));

pub static CARD: CardDef = CardDef { name: "Elda, Conjurer of Spectacle", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
