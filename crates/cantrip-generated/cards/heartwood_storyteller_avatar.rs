// Heartwood Storyteller Avatar
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// The first creature spell you cast each turn costs {1} less to cast.
// The first noncreature spell each opponent casts each turn costs {1} more to cast.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && (q.chars(spell).is(CardType::Creature) && q.history().this_turn().iter().filter_map(|r| match r.event { Event::Cast(cast) => Some(cast), _ => None }).take_while(|cast| cast.spell != spell).filter(|cast| Some(asker.controller) == Some(cast.player) && cast.chars(q).is(CardType::Creature)).count() as i64 + 1 == 1i64) { CostDelta::less((1i64).max(0) as u32) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player != asker.controller && (!(q.chars(spell).is(CardType::Creature)) && q.history().this_turn().iter().filter_map(|r| match r.event { Event::Cast(cast) => Some(cast), _ => None }).take_while(|cast| cast.spell != spell).filter(|cast| Some(player) == Some(cast.player) && !(cast.chars(q).is(CardType::Creature))).count() as i64 + 1 == 1i64) { CostDelta::more(1) } else { CostDelta::NONE } }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: -1, life: 4 }));

pub static CARD: CardDef = CardDef { name: "Heartwood Storyteller Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_0_0_rule,ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
