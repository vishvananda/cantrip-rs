// Cuombajj Witches
// status: planned
// requires: targets chosen by another player
// {T}: This creature deals 1 damage to any target and 1 damage to any target of an opponent's choice.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("targets-chosen-by-another-player" => #[allow(unused_variables)] fn target_chooser_0(q: &dyn Query, asker: &Asker) -> Vec<PlayerId> { Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() });
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY,planned_value!("targets chosen by another player", cantrip_mechanics::targets::ANY => cantrip_mechanics::targets::ANY.chosen_by_one_of(target_chooser_0))];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let mut batch_0: Vec<(ObjectId, Target, u16)> = Vec::new();
let amount_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; if let Some(source) = source_0 { for t in { let q: &dyn Query = g; t_0.clone() } { batch_0.push((source, t, amount_0_0)); } }
let amount_0_1 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; if let Some(source) = source_0 { for t in { let q: &dyn Query = g; t_1.clone() } { batch_0.push((source, t, amount_0_1)); } }
g.damage_together(&batch_0);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Cuombajj Witches", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{T}: This creature deals 1 damage to any target and 1 damage to any target of an opponent's choice.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
