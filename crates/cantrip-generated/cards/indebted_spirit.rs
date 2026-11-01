// Indebted Spirit
// status: planned
// requires: engine: no Afterlife
// requires: engine: no Afterlife granted
// Bestow {2}{W} (If you cast this card for its bestow cost, it's an Aura spell with enchant creature. It becomes a creature again if it's not attached.)
// Afterlife 1 (When this permanent is put into a graveyard from the battlefield, create a 1/1 white and black Spirit creature token with flying.)
// Enchanted creature gets +1/+1 and has afterlife 1.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(planned_value!("engine: no Afterlife granted", AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) => AbilityRef::granted_static({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))); } }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((1i64) as i16, (1i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
planned_item!("engine-no-afterlife" => abilities: cantrip_mechanics::afterlife::trigger::<1>("Afterlife 1"));
planned_item!("engine-no-afterlife" => extras: cantrip_mechanics::afterlife::SPIRIT);
planned_item!("engine-no-afterlife" => extras: cantrip_mechanics::afterlife::SPIRIT);
planned_item!("engine-no-afterlife-granted" => granted_rules: CardDef { keywords: Keywords::of(&[]), abilities: &[cantrip_mechanics::afterlife::trigger::<1>("Afterlife")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, ..CardDef::BLANK });

const FACE: CardDef = CardDef { name: "Indebted Spirit", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_2_apply_1, "Enchanted creature gets +1/+1 and has afterlife 1."),static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_2_apply_0, "Enchanted creature gets +1/+1 and has afterlife 1.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&BESTOWED];

pub static BESTOWED: CardDef = cantrip_mechanics::bestow::bestowed(FACE);
pub static CARD: CardDef = CardDef { bestow: Some(&BESTOWED), bestow_cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, ..FACE };
