// Midnight Mangler
// status: ready
// During turns other than yours, this Vehicle is an artifact creature.
// Crew 2 (Tap any number of creatures you control with total power 2 or more: This Vehicle becomes an artifact creature until end of turn.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).into_iter().any(|bound_0: PlayerId| Some(bound_0) == Some(q.state().turn.active))) && (source == affected) }
#[allow(unused_variables)] fn ability_0_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.types = chars.types.with(CardType::Artifact).with(CardType::Creature); }

pub static CARD: CardDef = CardDef { name: "Midnight Mangler", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::crew::crew(2, "Crew 2")], statics: &[static_ability(Layer::Type, ability_0_0_affects, ability_0_0_apply_0, "During turns other than yours, this Vehicle is an artifact creature.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
