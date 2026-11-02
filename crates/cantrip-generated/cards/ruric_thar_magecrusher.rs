// Ruric Thar, Magecrusher
// status: ready
// This spell can't be countered.
// Reach, vigilance, trample
// Ruric Thar has hexproof as long as they haven't dealt combat damage yet. (They can't be the target of spells or abilities your opponents control.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_4_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| !(q.history().all().iter().any(|r| matches!(r.event, Event::Damage { source, amount, kind, .. } if source == candidate && amount > 0 && kind.is_combat()))))) && (source == affected) }
#[allow(unused_variables)] fn ability_4_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Hexproof); }

pub static CARD: CardDef = CardDef { name: "Ruric Thar, Magecrusher", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Ogre,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CantBeCountered,Keyword::Reach,Keyword::Vigilance,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_4_0_affects, ability_4_0_apply_0, "Ruric Thar has hexproof as long as they haven't dealt combat damage yet.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
