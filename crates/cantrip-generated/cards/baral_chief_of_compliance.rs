// Baral, Chief of Compliance
// status: planned
// requires: trigger event Caused
// Instant and sorcery spells you cast cost {1} less to cast.
// Whenever a spell or ability you control counters a spell, you may draw a card. If you do, discard a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && (q.chars(spell).is(CardType::Instant) || q.chars(spell).is(CardType::Sorcery)) { CostDelta::less((1i64).max(0) as u32) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
cantrip_mechanics::effects::may_draw_then_discard(g, item)?;
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-caused" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Countered { spell } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; let data = [data[0], match q.event_cause() { Cause::Spell(c) | Cause::Ability { source: c, .. } => u32::from(q.controller(c).0), _ => u32::MAX }]; if true && match q.event_cause() { Cause::Spell(causer) | Cause::Ability { source: causer, .. } => Some(asker.controller) == Some(q.controller(causer)), _ => false } { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Baral, Chief of Compliance", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Caused", EventMask(0) => EventMask(EventMask::COUNTERED.0)), ability_1_event), ability_1, "Whenever a spell or ability you control counters a spell, you may draw a card. If you do, discard a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
