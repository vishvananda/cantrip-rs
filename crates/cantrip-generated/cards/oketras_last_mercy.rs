// Oketra's Last Mercy
// status: planned
// requires: duration DuringNextUntapStep
// requires: engine: no resolving rule on an open set: UntapStep
// Your life total becomes equal to your starting life total. Lands you control don't untap during your next untap step.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-resolving-rule-on-an-open-set-untap-step" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id))) });
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (Some(asker.controller).map_or(0, |player: PlayerId| i64::from(q.state().player(player).starting_life))).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for player in players_0 { g.set_life_total(player, amount_0); }
g.end_sentence();
let about_0 = Target::None; let until_0 = planned_value!("duration DuringNextUntapStep", Duration::Indefinite => Duration::DuringNextUntapStepOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }));
planned!("engine-no-resolving-rule-on-an-open-set-untap-step" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-untap-step" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "Lands you control don't untap during your next untap step.", rule: LastingRuleKind::DoesntUntap, applies: lasting_rule_0 }];);

pub static CARD: CardDef = CardDef { name: "Oketra's Last Mercy", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
