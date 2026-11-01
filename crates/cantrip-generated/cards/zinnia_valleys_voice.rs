// Zinnia, Valley's Voice
// status: planned
// requires: base power
// requires: player rule SpellsHaveAbilities
// Flying
// Zinnia gets +X/+0, where X is the number of other creatures you control with base power 1.
// Creature spells you cast gain offspring {2} as you cast them. (You may pay an additional {2} as you cast a creature spell. If you do, when that creature enters, create a 1/1 token copy of it.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = (((q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && (planned_value!("base power", 0i64 => q.base_power(id))) == (1i64))).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16, (0i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_2_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-spells-have-abilities" => statics: player_rule(ability_2_0_applies, PlayerRule::SpellsHave(SpellGrant { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| true, from: None, nth_each_turn: None, grants: &[SpellEffect::Offspring(ManaSymbol::ManaCostGeneric(2))] })));

pub static CARD: CardDef = CardDef { name: "Zinnia, Valley's Voice", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird,Subtype::Bard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::statics::itself, ability_1_apply_0, "Zinnia gets +X/+0, where X is the number of other creatures you control with base power 1.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
