// Celestial Archon
// status: ready
// Bestow {5}{W}{W} (If you cast this card for its bestow cost, it's an Aura spell with enchant creature. It becomes a creature again if it's not attached.)
// Flying, first strike
// Enchanted creature gets +4/+4 and has flying and first strike.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Flying); }
#[allow(unused_variables)] fn ability_3_apply_2(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::FirstStrike); }
#[allow(unused_variables)] fn ability_3_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((4i64) as i16, (4i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }

const FACE: CardDef = CardDef { name: "Celestial Archon", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Archon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::FirstStrike]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_3_apply_1, "Enchanted creature gets +4/+4 and has flying and first strike."),static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_3_apply_2, "Enchanted creature gets +4/+4 and has flying and first strike."),static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_3_apply_0, "Enchanted creature gets +4/+4 and has flying and first strike.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&BESTOWED];

pub static BESTOWED: CardDef = cantrip_mechanics::bestow::bestowed(FACE);
pub static CARD: CardDef = CardDef { bestow: Some(&BESTOWED), bestow_cost: ManaCost { generic: 5, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, ..FACE };
