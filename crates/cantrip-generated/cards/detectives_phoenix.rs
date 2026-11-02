// Detective's Phoenix
// status: planned
// requires: Bestow cost other than mana
// Bestow—{R}, Collect evidence 6. (To pay this bestow cost, pay {R} and exile cards with total mana value 6 or greater from your graveyard.)
// Flying, haste
// Enchanted creature gets +2/+2 and has flying and haste.
// You may cast this card from your graveyard using its bestow ability.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Flying);
chars.keywords = chars.keywords.with(Keyword::Haste); }
#[allow(unused_variables)] fn ability_3_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((2i64) as i16, (2i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
planned_item!("bestow-cost-other-than-mana" => ways: cantrip_mechanics::bestow::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { collect_evidence: 6, ..Cost::NONE }), "Bestow"));

pub static CARD: CardDef = CardDef { name: "Detective's Phoenix", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phoenix]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_3_apply_1, "Enchanted creature gets +2/+2 and has flying and haste."),static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_3_apply_0, "Enchanted creature gets +2/+2 and has flying and haste.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, bestow_from_graveyard: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
