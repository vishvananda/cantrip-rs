// Omnath, Locus of Mana
// status: planned
// requires: player rule RetainMana
// You don't lose unspent green mana as steps and phases end.
// Omnath gets +1/+1 for each unspent green mana you have.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).pool.get(Mana::G) + q.state().player(p).lasting.get(Mana::G)))).clamp(-32768, 32767) as i16, (Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).pool.get(Mana::G) + q.state().player(p).lasting.get(Mana::G)))).clamp(-32768, 32767) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
planned_item!("player-rule-retain-mana" => statics: player_rule(ability_0_0_applies, PlayerRule::RetainMana(ColorSet(16))));

pub static CARD: CardDef = CardDef { name: "Omnath, Locus of Mana", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::statics::itself, ability_1_apply_0, "Omnath gets +1/+1 for each unspent green mana you have.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
