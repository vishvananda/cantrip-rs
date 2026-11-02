// Court of Cunning
// status: ready
// When this enchantment enters, you become the monarch.
// At the beginning of your upkeep, any number of target players each mill two cards. If you're the monarch, each of those players mills ten cards instead. (To mill a card, a player puts the top card of their library into their graveyard.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.become_monarch(player); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, cantrip_api::TargetSpec::ANY_NUMBER)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if { let q: &dyn Query = g; Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).monarch) } {
let amount_1 = { let q: &dyn Query = g; (10i64).max(0) as u32 }; let mills_1: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; q.state().apnap().filter(|&p| t_0.iter().any(|t| t.player() == Some(p))).collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_1)).collect(); let milled_1 = g.mill_together(&mills_1);
g.end_sentence();
hoisted_2.extend(milled_1.clone());
hoisted_3.extend({ let q: &dyn Query = g; milled_1.clone() });
} else {
let amount_4 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let mills_4: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; q.state().apnap().filter(|&p| t_0.iter().any(|t| t.player() == Some(p))).collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_4)).collect(); let milled_4 = g.mill_together(&mills_4);
g.end_sentence();
hoisted_5.extend(milled_4.clone());
hoisted_6.extend({ let q: &dyn Query = g; milled_4.clone() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Court of Cunning", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this enchantment enters, you become the monarch.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, any number of target players each mill two cards. If you're the monarch, each of those players mills ten cards instead.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
