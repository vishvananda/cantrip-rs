// Living Breakthrough // Inventive Iteration
// status: planned
// requires: player rule for a while can't cast some spells
// Flying
// Whenever you cast a spell, your opponents can't cast spells with the same mana value as that spell until your next turn.
// (As this Saga enters and after your draw step, add a lore counter.)
// I — Return up to one target creature or planeswalker to its owner's hand.
// II — Return an artifact card from your graveyard to your hand. If you can't, draw a card.
// III — Exile this Saga, then return it to the battlefield transformed under your control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)))) }
static ABILITY_0_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_0_target_0).between(0, 1)];
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = g.return_to_hand(&objects_0);
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_2_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let selected_2 = if selected_2_candidates.len() as u64 >= u64::from(selected_2_count) { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::PutIntoHand, selected_2_candidates, cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::Exactly(selected_2_count), &[]))? } else { Vec::new() };
let objects_1 = { let q: &dyn Query = g; selected_2.clone() }; let moved_1 = g.return_to_hand(&objects_1);
let paid_1 = !objects_1.is_empty();
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_1 as i64) != 0)) } {
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_4 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_5 = ({ let q: &dyn Query = g; ((moved_4.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_5 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_5 { let controller = player_5; let entered = g.put_onto_battlefield_transformed(o, controller, false)?; if g.is_on_battlefield(entered) { moved.push(entered); } } moved };
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Inventive Iteration", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Return up to one target creature or planeswalker to its owner's hand.").targets(ABILITY_0_0_TARGETS),chapter(2, ability_0_1, "II — Return an artifact card from your graveyard to your hand. If you can't, draw a card.").targets(&[]),chapter(3, ability_0_2, "III — Exile this Saga, then return it to the battlefield transformed under your control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("player-rule-for-a-while-can-t-cast-some-spells" => let players_0_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_0_0 { g.add_player_effect(asker.controller, source, EffectKind::CantCast { player, spells: SpellSet::matching(move |q, spell| item.subject.object().is_some_and(|o: ObjectId| q.mana_value(spell) == q.mana_value(o))) }, Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") })); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Living Breakthrough", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Moonfolk]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(2)), power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Whenever you cast a spell, your opponents can't cast spells with the same mana value as that spell until your next turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
