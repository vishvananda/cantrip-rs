// Krang, Utrom Warlord
// status: ready
// Flying, trample, indestructible, haste
// Other artifact creatures you control have flying, trample, indestructible, and haste.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_4_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source) == Some(affected)) && ((*so_far).is(CardType::Artifact) && (*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_4_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Flying);
chars.keywords = chars.keywords.with(Keyword::Trample);
chars.keywords = chars.keywords.with(Keyword::Indestructible);
chars.keywords = chars.keywords.with(Keyword::Haste); }

pub static CARD: CardDef = CardDef { name: "Krang, Utrom Warlord", token_words: &[], oracle: true, cost: ManaCost { generic: 9, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Utrom,Subtype::Robot]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(9), toughness: Some(9), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample,Keyword::Indestructible,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_4_affects, ability_4_apply_0, "Other artifact creatures you control have flying, trample, indestructible, and haste.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
