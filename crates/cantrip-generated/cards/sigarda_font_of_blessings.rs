// Sigarda, Font of Blessings
// status: ready
// Flying
// Other permanents you control have hexproof.
// You may look at the top card of your library any time.
// You may cast Angel spells and Human spells from the top of your library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source) == Some(affected)) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Hexproof); }
#[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { (q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Angel)) || q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Human))) }

pub static CARD: CardDef = CardDef { name: "Sigarda, Font of Blessings", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_1_affects, ability_1_apply_0, "Other permanents you control have hexproof.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopSpells(ability_3_0_rule, None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
