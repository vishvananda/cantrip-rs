// Palace Jailer
// status: ready
// When this creature enters, you become the monarch.
// When this creature enters, exile target creature an opponent controls until an opponent becomes the monarch.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.become_monarch(player); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn until_1_crowned(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { match event { Event::BecameMonarch { player } if q.state().opponents(asker.controller).any(|o| o == *player) => Fired::PLAIN, _ => None } }
fn until_1_return(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { if let Target::Object(card) = item.subject { let owner = g.obj(card).owner; g.put_onto_battlefield(card, owner, false)?; } Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let entered_1 = g.history().all().iter().find(|r| matches!(r.event, Event::ZoneChange { new, to, .. } if new == item.object && to.is(ZoneKind::Battlefield))).map(|r| r.serial); let over_1 = entered_1.is_some_and(|since| g.history().all().iter().any(|r| r.serial > since && matches!(r.event, Event::BecameMonarch { player } if g.state().opponents(item.controller).any(|o| o == player)))); let moved_1 = match over_1 { false => { let by_1 = g.linked(item); g.exile_with(&objects_1, by_1, false) } true => Vec::new() }; for &card in &moved_1 { g.delayed_trigger(DelayedWhen::UntilEvent, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::Object(card)); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Palace Jailer", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, you become the monarch.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, exile target creature an opponent controls until an opponent becomes the monarch.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask::BECAME_MONARCH, until_1_crowned), until_1_return, "Exile target creature an opponent controls until an opponent becomes the monarch.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
