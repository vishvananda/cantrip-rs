// Starscream, Power Hungry // Starscream, Seeker Leader
// status: planned
// requires: engine: no LivingMetal
// requires: engine: no MoreThanMeetsTheEye
// Living metal (During your turn, this Vehicle is also a creature.)
// Flying, menace, haste
// Whenever Starscream deals combat damage to a player, if there is no monarch, that player becomes the monarch.
// Whenever you become the monarch, convert Starscream.
// More Than Meets the Eye {2}{B} (You may cast this card converted for {2}{B}.)
// Flying
// Whenever you draw a card, if you're the monarch, target opponent loses 2 life.
// Whenever one or more creatures deal combat damage to you, convert Starscream.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
fn ability_2_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).monarch) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Drew { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && q.chars(source).is(CardType::Creature) && matches!(*target, Target::Player(p) if Some(asker.controller) == Some(p)) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-more-than-meets-the-eye" => ways: cantrip_mechanics::more_than_meets_the_eye::way(ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "More Than Meets the Eye {2}{B}"));

pub static CARD: CardDef = CardDef { name: "Starscream, Power Hungry", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Robot]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DREW.0), ability_2_event), ability_2, "Whenever you draw a card, if you're the monarch, target opponent loses 2 life.").targets(ABILITY_2_TARGETS).condition(ability_2_if),triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_3_event), ability_3, "Whenever one or more creatures deal combat damage to you, convert Starscream.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_4_if(q: &dyn Query, asker: &Asker) -> bool { (({ let mut all = q.state().apnap().collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|player: PlayerId| q.state().player(player).monarch)).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all }).len() as i64) == (0i64) }
#[allow(unused_variables)] fn ability_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).expect("a reference that always names a player") }; g.become_monarch(player); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::BecameMonarch { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-living-metal" => statics: cantrip_mechanics::living_metal::ON_YOUR_TURN);

pub static CARD: CardDef = CardDef { name: "Starscream, Seeker Leader", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(4)), power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Menace,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_4_event), ability_4, "Whenever Starscream deals combat damage to a player, if there is no monarch, that player becomes the monarch.").targets(&[]).condition(ability_4_if),triggered(when_asked(EventMask(EventMask::BECAME_MONARCH.0), ability_5_event), ability_5, "Whenever you become the monarch, convert Starscream.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
