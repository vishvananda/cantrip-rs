// Caretaker's Talent
// status: ready
// (Gain the next level as a sorcery to add its ability.)
// Whenever one or more tokens you control enter, draw a card. This ability triggers only once each turn.
// {W}: Level 2
// When this Class becomes level 2, create a token that's a copy of target token you control.
// {3}{W}: Level 3
// Creature tokens you control get +2/+2.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_level_2_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_1_LEVEL_2_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_level_2_0_target_0)];
#[allow(unused_variables)] fn ability_1_level_2_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ClassLevel { object, level } = event { let id = *object; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) && *level == (2i64).max(0) as u8 && q.state().obj(asker.source).class_level >= 2 { return Some(Fired { subject, data }); } }
 None }
fn ability_1_level_2_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_1 = match original_1 { Some(original) => g.create_token_copies(original, player_1, count_1)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_level_3_0_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().obj(source).class_level >= 3) && (((*so_far).is(CardType::Creature) && q.state().obj(affected).token && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_1_level_3_0_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((2i64) as i16, (2i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Caretaker's Talent", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Class]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Whenever one or more tokens you control enter, draw a card. This ability triggers only once each turn.").targets(&[]).once_per_batch().triggers_each_turn(1),cantrip_mechanics::class::level_two("{W}", "{W}: Level 2"),triggered(when_asked(EventMask(EventMask::CLASS_LEVEL.0), ability_1_level_2_0_event), ability_1_level_2_0, "Creature tokens you control get +2/+2.").targets(ABILITY_1_LEVEL_2_0_TARGETS),cantrip_mechanics::class::level_three("{3}{W}", "{3}{W}: Level 3")], statics: &[static_ability(Layer::PtModify, ability_1_level_3_0_0_affects, ability_1_level_3_0_0_apply_0, "Creature tokens you control get +2/+2.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
