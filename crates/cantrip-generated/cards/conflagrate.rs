// Conflagrate
// status: planned
// requires: discarding X cards as a cost
// requires: numeric expression AnnouncedX
// requires: target cardinality UpTo
// Conflagrate deals X damage divided as you choose among any number of targets.
// Flashback—{R}{R}, Discard X cards. (You may cast this card from your graveyard for its flashback cost. Then exile it.)
#[allow(unused_imports)] use cantrip_api::*;
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY.between(1, planned_value!("target cardinality UpTo", 1u32 => TargetSpec::read_as_chosen(|q: &dyn Query, asker: &Asker| (planned_value!("numeric expression AnnouncedX", 0i64 => i64::from(q.announced_x().unwrap_or(0)))).max(0) as u32)))];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
cantrip_mechanics::damage_divided(g, item);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_1(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { true }
#[allow(unused_variables)] fn ability_1_require(q: &dyn Query, asker: &Asker) -> bool { (q.chars(asker.source).is(CardType::Instant) || q.chars(asker.source).is(CardType::Sorcery)) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Conflagrate", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 2, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("discarding X cards as a cost", Cost::NONE => Cost { variable: Some(VariableCost::DiscardX(ability_1_1)), ..Cost::NONE })), ZoneKind::Graveyard, "Flashback—{R}{R}, Discard X cards.").only_from().exile_after().condition(ability_1_require)], divide: Some(Divide { total: TargetSpec::COUNTED, of_x: None }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
