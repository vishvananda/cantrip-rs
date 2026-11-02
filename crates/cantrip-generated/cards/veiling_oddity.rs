// Veiling Oddity
// status: planned
// requires: engine: no event for a counter being removed
// requires: engine: no resolving rule on an open set: Prohibit BeBlocked
// Suspend 4—{1}{U} (Rather than cast this card from your hand, you may pay {1}{U} and exile it with four time counters on it. At the beginning of your upkeep, remove a time counter. When the last is removed, you may cast it without paying its mana cost. It has haste.)
// When the last time counter is removed from this card while it's exiled, creatures can't be blocked this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; q.chars(id).is(CardType::Creature) });
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = Target::None; let until_0 = Duration::EndOfTurn;
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "When the last time counter is removed from this card while it's exiled, creatures can't be blocked this turn.", rule: LastingRuleKind::CantBeBlocked, applies: lasting_rule_0 }];);
planned_item!("engine-no-event-for-a-counter-being-removed" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::CountersRemoved { object, kind, amount, by, .. } = event { let id = *object; let subject = Target::Object(id); let data = [*amount as u32, id.0]; if Some(asker.source) == Some(id) && *kind == CounterKind::Time && q.state().obj(id).counters(*kind) == 0 { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Veiling Oddity", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Illusion]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::suspend::ability(ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Suspend"),triggered(when_asked(planned_value!("engine: no event for a counter being removed", EventMask(0) => EventMask(EventMask::COUNTERS_REMOVED.0)), ability_1_event), ability_1, "When the last time counter is removed from this card while it's exiled, creatures can't be blocked this turn.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Exile]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, suspend: Some(SuspendDef { count: 4, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] } }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
