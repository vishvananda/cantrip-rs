// Extraordinary Journey
// status: planned
// requires: duration WhileAnyExiled
// requires: play permission on an open set of cards
// When this enchantment enters, exile up to X target creatures. For each of those cards, its owner may play it for as long as it remains exiled.
// Whenever one or more nontoken creatures enter, if one or more of them entered from exile or was cast from exile, you draw a card. This ability triggers only once each turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, TargetSpec::COUNTED)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
g.one_instruction(&mut |g| { for (index_1, bound_0) in players_1.iter().copied().enumerate() { planned!("play-permission-on-an-open-set-of-cards" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::Play { zone: ZoneKind::Exile, cards: CardsOfRule::kept(move |q: &dyn Query, asker: &Asker, card: ObjectId| (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((moved_0.clone()).contains(&id) && Some(bound_0) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>()).contains(&card)), grant: PlayGrant { lands: true, ..PlayGrant::CAST } }), planned_value!("duration WhileAnyExiled", Duration::Indefinite => Duration::WhileAnyExiled({ let q: &dyn Query = g; moved_0.clone() }))); } });
    } Ok(()) })?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && from.is(ZoneKind::Exile) && (!(q.state().obj(id).token) && q.chars(id).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && ((!(q.state().obj(id).token) && q.chars(id).is(CardType::Creature)) && cast_at(q, event, id).is_some_and(|cast| cast.from == ZoneKind::Exile)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn cast_of<'a>(q: &'a dyn Query, id: ObjectId) -> Option<&'a Cast> { let records = q.history().all(); let spell = records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == id => Some(old), _ => None }).unwrap_or(id); records.iter().rev().find_map(|r| match &r.event { Event::Cast(cast) if cast.spell == spell => Some(cast), _ => None }) }
#[allow(dead_code)] fn cast_at<'a>(q: &'a dyn Query, event: &Event, id: ObjectId) -> Option<&'a Cast> { match event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if *new == id => cast_of(q, *old), _ => cast_of(q, id) } }

pub static CARD: CardDef = CardDef { name: "Extraordinary Journey", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 2, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this enchantment enters, exile up to X target creatures. For each of those cards, its owner may play it for as long as it remains exiled.").targets(ABILITY_0_TARGETS),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever one or more nontoken creatures enter, if one or more of them entered from exile or was cast from exile, you draw a card. This ability triggers only once each turn.").targets(&[]).triggers_each_turn(1)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
