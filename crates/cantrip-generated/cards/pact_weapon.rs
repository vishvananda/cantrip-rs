// Pact Weapon
// status: ready
// As long as this Equipment is attached to a creature, you don't lose the game for having 0 or less life.
// Whenever equipped creature attacks, draw a card and reveal it. The creature gets +X/+X until end of turn and you lose X life, where X is that card's mana value.
// Equip—Discard a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, reason: LossReason) -> bool { (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).attached_to.is_some_and(|host| (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).contains(&host)))) && player == asker.controller && matches!(reason, LossReason::Life) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mut done_0: i64 = 0; let mut drawn_0: Vec<ObjectId> = Vec::new(); for player in players_0 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_0); done_0 += g.history().drawn(player) as i64 - before; drawn_0.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; drawn_0.clone() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::Pump((((drawn_0.clone()).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).clamp(-32768, 32767) as i16).max(0), (((drawn_0.clone()).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).clamp(-32768, 32767) as i16).max(0)) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; ((drawn_0.clone()).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).max(0) as i32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.lose_life(player, amount_3); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let attachments_4 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_4 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; if let Some(host) = host_4 { cantrip_api::instruction(g, |g| { for attachment in attachments_4 { g.attach(attachment, host); } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Pact Weapon", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Equipment]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever equipped creature attacks, draw a card and reveal it. The creature gets +X/+X until end of turn and you lose X life, where X is that card's mana value.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(discard_cards(1)), ability_2, "Equip—Discard a card.").targets(ABILITY_2_TARGETS).equip().sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, loss_rules: &[ability_0_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
