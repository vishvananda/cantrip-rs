// Takeno, Samurai General
// status: planned
// requires: engine: no Bushido
// requires: numeric object property BushidoValue
// Bushido 2 (Whenever this creature blocks or becomes blocked, it gets +2/+2 until end of turn.)
// Each other Samurai creature you control gets +1/+1 for each point of bushido it has.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source) == Some(affected)) && ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Samurai)) && (*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = (((Some(affected).map_or(0i64, |o: ObjectId| planned_value!("numeric object property BushidoValue", 0i64 => i64::from(q.chars(o).bushido)))).clamp(-32768, 32767) as i16).max(0), ((Some(affected).map_or(0i64, |o: ObjectId| planned_value!("numeric object property BushidoValue", 0i64 => i64::from(q.chars(o).bushido)))).clamp(-32768, 32767) as i16).max(0)); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
planned_item!("engine-no-bushido" => abilities: cantrip_mechanics::bushido::trigger::<2>("Bushido 2"));

pub static CARD: CardDef = CardDef { name: "Takeno, Samurai General", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Samurai]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability_for_each(Layer::PtModify, ability_1_affects, ability_1_apply_0, "Each other Samurai creature you control gets +1/+1 for each point of bushido it has.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
