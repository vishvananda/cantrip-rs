// Stormtide Leviathan
// status: ready
// Islandwalk (This creature can't be blocked as long as defending player controls an Island.)
// All lands are Islands in addition to their other types.
// Creatures without flying or islandwalk can't attack.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (*so_far).is(CardType::Land) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.subtypes = chars.subtypes.with(q.words(asker.source).subtype(Subtype::Island)); }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && !((*so_far).keywords.has(Keyword::Flying)) && !((*so_far).keywords.has(Keyword::Islandwalk))) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { let source = asker.source; let asker = &Asker::new(source, q.controller(source)); !((q.chars(attacker).is(CardType::Creature) && !(q.chars(attacker).keywords.has(Keyword::Flying)) && !(q.chars(attacker).keywords.has(Keyword::Islandwalk)))) }

pub static CARD: CardDef = CardDef { name: "Stormtide Leviathan", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Leviathan]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(8), toughness: Some(8), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Islandwalk]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Type, ability_1_affects, ability_1_apply_0, "All lands are Islands in addition to their other types.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[ability_2_0_rule], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
