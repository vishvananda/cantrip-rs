// High Fae Prankster
// status: planned
// requires: engine: no perpetual effects
// requires: operation Exchange
// Flying, Deathtouch
// When this creature enters, choose up to one —
// • Perpetually exchange target creature's base power with another target creature's base power.
// • This creature perpetually has base power and toughness 4/1.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn ability_2_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_2_mode_0_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chosen_targets().get(0).copied().and_then(|t: Target| t.object()) == Some(id)) && q.chars(id).is(CardType::Creature)))) }
static ABILITY_2_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_mode_0_target_0),target(ability_2_mode_0_target_1)];
fn ability_2_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
planned!("operation-exchange" => cantrip_mechanics::exchange::exchange(g, asker.source, Exchange::Power, { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }, { let q: &dyn Query = g; t_1_raw.get(0).copied().and_then(|t: Target| t.object()) }, None));
g.end_sentence();
 Ok(()) }
fn ability_2_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_2 { let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::SetPt((4i64) as i16, (1i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); });
g.end_sentence();
 Ok(()) }
static ABILITY_2_MODES: ModalDef = modes(&[mode(ability_2_mode_0, "Perpetually exchange target creature's base power with another target creature's base power.").targets(ABILITY_2_MODE_0_TARGETS),mode(ability_2_mode_1, "This creature perpetually has base power and toughness 4/1.").targets(&[])]).choose(0, 1);
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "High Fae Prankster", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Faerie,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), &ABILITY_2_MODES, "When this creature enters, choose up to one — Perpetually exchange target creature's base power with another target creature's base power; or This creature perpetually has base power and toughness 4/1.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
