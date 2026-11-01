// Akiri, Line-Slinger
// status: ready
// First strike, vigilance
// Akiri gets +1/+0 for each artifact you control.
// Partner (You can have two commanders if both have partner.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = (((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16, ((0i64 * (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64)) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }

pub static CARD: CardDef = CardDef { name: "Akiri, Line-Slinger", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Kor,Subtype::Soldier,Subtype::Ally]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(0), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike,Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::statics::itself, ability_2_apply_0, "Akiri gets +1/+0 for each artifact you control.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
