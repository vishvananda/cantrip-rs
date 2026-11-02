// Switchgrass Grazer
// status: planned
// requires: engine: no perpetual effects
// Haste
// Whenever this creature attacks, it deals 1 damage to any target. If this creature is saddled and a creature was dealt damage this way, that creature perpetually gains "This creature can't block" and "Damage isn't removed from this creature during cleanup steps."
// Saddle 1
#[allow(unused_imports)] use cantrip_api::*;
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
static PERPETUAL_3: CardDef = CardDef { keywords: Keywords::of(&[Keyword::CantBlock]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, ..CardDef::BLANK };
#[allow(unused_variables)] fn perpetual_4_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { id == asker.source }
static PERPETUAL_4: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, damage_stays: &[perpetual_4_0_rule], ..CardDef::BLANK };
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut done_0: i64 = 0; let mut dealt_0: Vec<ObjectId> = Vec::new(); let mut hurt_0: Vec<PlayerId> = Vec::new(); let before_0 = g.history().head() as usize; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); done_0 += g.damage_together(&batch).iter().map(|&n| i64::from(n)).sum::<i64>(); dealt_0.extend(cantrip_mechanics::history::permanents_dealt_damage_by(g.history(), source, before_0)); for r in &g.history().all()[before_0..] { if let Event::Damage { source: s, target: Target::Player(p), amount, .. } = r.event { if s == source && amount > 0 && !hurt_0.contains(&p) { hurt_0.push(p); } } } }
g.end_sentence();
if { let q: &dyn Query = g; (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).marks.saddled_turn == q.state().turn.number) && ((q.state().battlefield().iter().copied().filter(|&id| (dealt_0.contains(&id) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64)) } {
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && dealt_0.contains(&id))).collect::<Vec<ObjectId>>() };
let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_3) };
let kind_2_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_4) };
for &object in &objects_2 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_1); });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Switchgrass Grazer", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Horse,Subtype::Mount]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, it deals 1 damage to any target. If this creature is saddled and a creature was dealt damage this way, that creature perpetually gains \"This creature can't block\" and \"Damage isn't removed from this creature during cleanup steps.\"").targets(ABILITY_1_TARGETS),cantrip_mechanics::saddle::ability(1, "Saddle")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
