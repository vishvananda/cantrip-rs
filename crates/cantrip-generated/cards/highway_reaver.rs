// Highway Reaver
// status: planned
// requires: Unearth cost other than mana
// requires: a keyword's cost changed
// requires: engine: no cost copies a mana cost
// requires: engine: no perpetual effects
// requires: object filter HasKeywordFamily
// Flying
// Start your engines!
// Whenever this creature enters or attacks, target creature card in your graveyard without unearth perpetually gains unearth. The unearth cost is equal to its mana cost.
// Max speed — You may pay {0} rather than pay the unearth cost of the first unearth ability you activate each turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner)) && !(planned_value!("object filter HasKeywordFamily", false => q.has_keyword_ability(id, KeywordAbility::Unearth)))))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
static PERPETUAL_2: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, ..CardDef::BLANK };
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("engine-no-perpetual-effects" => let objects_1: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() };
for &object in &objects_1 { let kind_1_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_2) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_1_0); });
g.end_sentence();
 Ok(()) }
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_3_0_keyword_cards_0(q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId) -> bool { player == asker.controller && true });
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_3_0_keyword_per_0(q: &dyn Query, asker: &Asker) -> i64 { 1 });
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("unearth-cost-other-than-mana" => ways: cantrip_mechanics::unearth::way_with_cost(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("engine: no cost copies a mana cost", Cost::NONE => Cost { own_mana_cost: true, ..Cost::NONE })), "Unearth"));
planned_item!("a-keyword-s-cost-changed" => keyword_costs: KeywordCostRule { keyword: CostedKeyword::Unearth, applies: ability_3_0_keyword_cards_0, change: KeywordCostChange::Alternate(Cost::generic(0)), per: ability_3_0_keyword_per_0, first_each_turn: true });

pub static CARD: CardDef = CardDef { name: "Highway Reaver", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Scorpion,Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::ATTACKS.0), ability_2_event), ability_2, "Whenever this creature enters or attacks, target creature card in your graveyard without unearth perpetually gains unearth. The unearth cost is equal to its mana cost.").targets(ABILITY_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, start_your_engines: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
