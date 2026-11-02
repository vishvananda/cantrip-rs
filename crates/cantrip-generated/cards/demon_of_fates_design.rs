// Demon of Fate's Design
// status: planned
// requires: engine: no cost sized by the game
// requires: object reference Scoped
// requires: spell cost rule with terms
// Flying, trample
// Once during each of your turns, you may cast an enchantment spell by paying life equal to its mana value rather than paying its mana cost.
// {2}{B}, Sacrifice another enchantment: This creature gets +X/+0 until end of turn, where X is the sacrificed enchantment's mana value.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-cost-sized-by-the-game" => #[allow(unused_variables)] fn rule_cost_0_cost_0(q: &dyn Query, asker: &Asker, targets: &[Target]) -> Cost { Cost { life: (planned_value!("object reference Scoped", None::<ObjectId> => q.scoped_object(asker)).map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).clamp(0, i64::from(u32::MAX)) as u32, ..Cost::NONE } });
#[allow(unused_variables)] fn ability_2_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::Pump((((q.cost_objects(item).to_vec()).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).clamp(-32768, 32767) as i16).max(0), (0i64) as i16) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_2(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Enchantment)) }
planned_item!("spell-cost-rule-with-terms" => statics: player_rule(ability_2_0_applies, PlayerRule::SpellCost { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| q.chars(spell).is(CardType::Enchantment), change: CostModifier::Alternate { cost: PlayCost::Alternate(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("engine: no cost sized by the game", Cost::NONE => Cost::reading(rule_cost_0_cost_0)))), limit: Some(PlayLimit::EachTurn(1)), during: Some(|q: &dyn Query, asker: &Asker, player: PlayerId| player == asker.controller) } }));

pub static CARD: CardDef = CardDef { name: "Demon of Fate's Design", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(1, ability_3_2)), ability_3, "{2}{B}, Sacrifice another enchantment: This creature gets +X/+0 until end of turn, where X is the sacrificed enchantment's mana value.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
