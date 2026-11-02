// Spoils of War
// status: planned
// requires: a limit on the X announced
// X is the number of artifact and/or creature cards in an opponent's graveyard as you cast this spell.
// Distribute X +1/+1 counters among any number of target creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, TargetSpec::COUNTED)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
cantrip_mechanics::counters_divided(g, item, CounterKind::PlusOne);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("a-limit-on-the-x-announced", "(kind:Casting,args:Require((kind:Compare,args:(left:(kind:AnnouncedX),comparison:Equal,right:(kind:CountObjects,args:(kind:Zone,args:(scope:Players(zone:Graveyard,players:(kind:One,args:(kind:OneOf,args:(kind:OpponentsOf,args:(kind:AbilityController))))),kind:Card,filter:(kind:Either,args:[(kind:CardType,args:Artifact),(kind:CardType,args:Creature)]))))))))");

pub static CARD: CardDef = CardDef { name: "Spoils of War", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, divide: Some(Divide { total: TargetSpec::COUNTED, of_x: None }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
