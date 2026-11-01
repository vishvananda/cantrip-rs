// Barbarian Class
// status: planned
// requires: engine: no event for dice rolled together
// requires: replacement of RollDice
// (Gain the next level as a sorcery to add its ability.)
// If you would roll one or more dice, instead roll that many dice plus one and ignore the lowest roll.
// {1}{R}: Level 2
// Whenever you roll one or more dice, target creature you control gets +2/+0 and gains menace until end of turn.
// {2}{R}: Level 3
// Creatures you control have haste.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_level_2_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_1_LEVEL_2_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_level_2_0_target_0)];
#[allow(unused_variables)] fn ability_1_level_2_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1_level_2_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::Pump((2i64) as i16, (0i64) as i16) };
let kind_0_1_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Menace])) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0); g.add_effect(source, object, kind_0_1_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_level_3_0_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().obj(source).class_level >= 3) && (((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_1_level_3_0_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Haste); }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("replacement-of-roll-dice" => callbacks: #[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::DieRoll(p) = proposal { let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if Some(asker.controller) == Some(p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::DieRoll(seen) = &seen { let p = seen; let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if let Proposal::DieRoll(p) = proposal { p.extra_dice += 1; p.ignore = Some(DiceIgnore::Lowest); } } }
);
planned_item!("replacement-of-roll-dice" => replacements: replacement(ReplacementKind::DieRoll, ability_0_applies, ability_0_apply, "If you would roll one or more dice, instead roll that many dice plus one and ignore the lowest roll."));
planned_item!("engine-no-event-for-dice-rolled-together" => matcher: #[allow(unused_variables)] fn ability_1_level_2_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::DiceRolled { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) && q.state().obj(asker.source).class_level >= 2 { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Barbarian Class", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Class]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::class::level_two("{1}{R}", "{1}{R}: Level 2"),triggered(when_asked(planned_value!("engine: no event for dice rolled together", EventMask(0) => EventMask(EventMask::DICE_ROLLED.0)), ability_1_level_2_0_event), ability_1_level_2_0, "If you would roll one or more dice, instead roll that many dice plus one and ignore the lowest roll.").targets(ABILITY_1_LEVEL_2_0_TARGETS),cantrip_mechanics::class::level_three("{2}{R}", "{2}{R}: Level 3")], statics: &[static_ability(Layer::Ability, ability_1_level_3_0_0_affects, ability_1_level_3_0_0_apply_0, "If you would roll one or more dice, instead roll that many dice plus one and ignore the lowest roll.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
