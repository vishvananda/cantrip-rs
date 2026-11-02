// Spider-Punk
// status: planned
// requires: engine: no Riot granted
// requires: static rule Prohibit Counter of abilities
// Riot (This creature enters with your choice of a +1/+1 counter or haste.)
// Other Spiders you control have riot.
// Spells and abilities can't be countered.
// Damage can't be prevented.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_riot(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { cantrip_mechanics::riot::enters(g, entering, &CARD) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source) == Some(affected)) && ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Spider)) && Some(asker.controller) == Some(q.controller(affected)))) }
fn granted_0_riot(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { cantrip_mechanics::riot::enters(g, entering, &CARD) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(planned_value!("engine: no Riot granted", AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) => AbilityRef::granted_static({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))); } }
#[allow(unused_variables)] fn ability_3_unpreventable(q: &dyn Query, asker: &Asker, source: ObjectId, combat: bool) -> bool { true }
planned_item!("engine-no-riot-granted" => granted_rules: CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[cantrip_mechanics::riot::HASTE_STATIC], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: Some(granted_0_riot), labels: cantrip_mechanics::riot::LABELS, ..CardDef::BLANK });
planned_item!("static-rule-prohibit-counter-of-abilities" => abilities_cant_be_countered: true);

pub static CARD: CardDef = CardDef { name: "Spider-Punk", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spider,Subtype::Human,Subtype::Hero]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[cantrip_mechanics::riot::HASTE_STATIC,static_ability(Layer::Ability, ability_1_affects, ability_1_apply_0, "Other Spiders you control have riot.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_riot), modal: None, enchant: None, unpreventable_damage: &[ability_3_unpreventable], labels: cantrip_mechanics::riot::LABELS, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
