// Spirit of the Night
// status: ready
// Flying, trample, haste, protection from black
// Spirit of the Night has first strike as long as it's attacking.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_4_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().attackers.iter().any(|a| a.id == candidate))) && (source == affected) }
#[allow(unused_variables)] fn ability_4_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::FirstStrike); }

pub static CARD: CardDef = CardDef { name: "Spirit of the Night", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 3, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon,Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample,Keyword::Haste]), protection: ColorSet(4), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_4_0_affects, ability_4_0_apply_0, "Spirit of the Night has first strike as long as it's attacking.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
