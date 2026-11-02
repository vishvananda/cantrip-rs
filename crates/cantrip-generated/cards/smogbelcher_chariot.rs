// Smogbelcher Chariot
// status: planned
// requires: engine: no perpetual effects
// Menace, Deathtouch, Lifelink
// Whenever this Vehicle attacks, target creature that crewed it this turn perpetually gains your choice of menace, deathtouch, or lifelink.
// Crew 3
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Crewed { creature, vehicle } = event { let id = *creature; let other = *vehicle; let subject = Target::Object(id); let data = [other.0, u32::MAX]; if Some(id) == Some(id) && q.event_subject().object() == Some(other) { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64)))) }
static ABILITY_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_3_target_0)];
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let chosen_0 = cantrip_mechanics::choose::option(g, player_0, asker.source, item.ability.card, 0..3)? - 0;
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() };
for &object in &objects_2 { let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(match chosen_0 { 0 => EffectKind::AddKeywords(Keywords::of(&[Keyword::Menace])), 1 => EffectKind::AddKeywords(Keywords::of(&[Keyword::Deathtouch])), _ => EffectKind::AddKeywords(Keywords::of(&[Keyword::Lifelink])), }) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Smogbelcher Chariot", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace,Keyword::Deathtouch,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_3_event), ability_3, "Whenever this Vehicle attacks, target creature that crewed it this turn perpetually gains your choice of menace, deathtouch, or lifelink.").targets(ABILITY_3_TARGETS),cantrip_mechanics::crew::crew(3, "Crew 3")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["menace", "deathtouch", "lifelink"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
