// Vazi, Keen Negotiator
// status: ready
// Haste
// {T}: Target opponent creates X Treasure tokens, where X is the number of Treasure tokens you created this turn.
// Whenever an opponent casts a spell or activates an ability, if mana from a Treasure was spent to cast it or activate it, put a +1/+1 counter on target creature, then draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: token!(Treasure).name, token_words: token!(Treasure).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Treasure]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "Add mana.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let players_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::ZoneChange { new, to, reason: ZoneReason::Created, .. } = event { let id = *new; let player = q.controller(id); let subject = Target::Object(id); let data = [new.0, u32::from(player.0)]; if to.is(ZoneKind::Battlefield) && Some(asker.controller) == Some(player) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Treasure)) { return Some(Fired { subject, data }); } }  None }).count() as i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = Words::NONE;
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_or_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_2_OR_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_or_0_target_0)];
#[allow(unused_variables)] fn ability_2_or_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&*player) && true && subject.object().is_some_and(|candidate: ObjectId| cast_at(q, event, candidate).is_some_and(|cast| q.history().mana_spent_to_cast(cast.spell).any(|(source, _)| q.chars(source).has_subtype(q.words(asker.source).subtype(Subtype::Treasure))))) { return Some(Fired { subject, data }); } }
 None }
fn ability_2_or_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| cast_of(q, candidate).is_some_and(|cast| q.history().mana_spent_to_cast(cast.spell).any(|(source, _)| q.chars(source).has_subtype(q.words(asker.source).subtype(Subtype::Treasure))))) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_2.push((object, CounterKind::PlusOne, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_or_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_2_OR_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_or_1_target_0)];
#[allow(unused_variables)] fn ability_2_or_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AbilityActivated { source, ability, player, .. } = event { let id = *source; let def = q.activated_ability(*ability); let subject = Target::Player(*player); let serial = match cantrip_mechanics::triggers::activated_ability_on_stack(q, event) { Some(Target::Ability(serial)) => serial, _ => 0 }; let data = [id.0, serial]; if def.is_some() && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&*player) && true && q.state().stack.iter().find(|i| serial != 0 && i.serial == serial && i.kind == StackKind::Activated).is_some_and(|i| q.history().all().get(i.since as usize..).unwrap_or(&[]).iter().filter_map(|r| match r.event { Event::ManaSpent { on, cast: false, source, mana, .. } if on == id => Some((source, mana)), _ => None }).any(|(source, _)| source != ObjectId::NONE && q.chars(source).has_subtype(q.words(asker.source).subtype(Subtype::Treasure)))) { return Some(Fired { subject, data }); } }
 None }
fn ability_2_or_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut counters_4: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_4.push((object, CounterKind::PlusOne, amount)); } }
let had_4: Vec<_> = counters_4.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_4); let applied_4: Vec<ObjectId> = counters_4.iter().zip(had_4).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.draw(player, amount_5); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn cast_of<'a>(q: &'a dyn Query, id: ObjectId) -> Option<&'a Cast> { let records = q.history().all(); let spell = records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == id => Some(old), _ => None }).unwrap_or(id); records.iter().rev().find_map(|r| match &r.event { Event::Cast(cast) if cast.spell == spell => Some(cast), _ => None }) }
#[allow(dead_code)] fn cast_at<'a>(q: &'a dyn Query, event: &Event, id: ObjectId) -> Option<&'a Cast> { match event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if *new == id => cast_of(q, *old), _ => cast_of(q, id) } }

pub static CARD: CardDef = CardDef { name: "Vazi, Keen Negotiator", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Advisor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{T}: Target opponent creates X Treasure tokens, where X is the number of Treasure tokens you created this turn.").targets(ABILITY_1_TARGETS),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_2_or_0_event), ability_2_or_0, "Whenever an opponent casts a spell or activates an ability, if mana from a Treasure was spent to cast it or activate it, put a +1/+1 counter on target creature, then draw a card.").targets(ABILITY_2_OR_0_TARGETS), triggered(when_asked(EventMask(EventMask::ABILITY_ACTIVATED.0), ability_2_or_1_event), ability_2_or_1, "Whenever an opponent casts a spell or activates an ability, if mana from a Treasure was spent to cast it or activate it, put a +1/+1 counter on target creature, then draw a card.").targets(ABILITY_2_OR_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
