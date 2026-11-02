// Haunting Misery
// status: planned
// requires: engine: no cost moves these objects
// As an additional cost to cast this spell, exile X creature cards from your graveyard.
// Haunting Misery deals X damage to target player or planeswalker.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { match t { Target::Player(p) => true, Target::Object(id) => q.is_on_battlefield(id) && { let c = q.chars(id); c.is(CardType::Creature) || c.is(CardType::Planeswalker) || c.is(CardType::Battle) } && (q.chars(id).is(CardType::Planeswalker)), _ => false } }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let amount_1 = { let q: &dyn Query = g; (i64::from(item.x)) as u16 }; let recipients_1 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_1 { let batch: Vec<(ObjectId, Target, u16)> = recipients_1.iter().map(|t| (source, *t, amount_1)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Haunting Misery", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("engine: no cost moves these objects", Cost::NONE => Cost { moves: Some(MovesObjects { from: CostZone::Graveyard(Whose::Yours), to: CostDestination::Exile, count: CostCount::Announced, filter: ability_0_0 }), ..Cost::NONE }))), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
