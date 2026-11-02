// Meteor Shower
// status: ready
// Meteor Shower deals X plus 1 damage divided as you choose among any number of targets.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_divided_total(x: u16) -> u16 { let x = i64::from(x); ((x + 1i64)).clamp(0, i64::from(u16::MAX)) as u16 }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY.between(1, cantrip_api::TargetSpec::ANY_NUMBER)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
cantrip_mechanics::damage_divided(g, item);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Meteor Shower", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 2, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, divide: Some(Divide { total: TargetSpec::COUNTED, of_x: Some(ability_0_divided_total) }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
