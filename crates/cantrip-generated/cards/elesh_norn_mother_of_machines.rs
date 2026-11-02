// Elesh Norn, Mother of Machines
// status: planned
// requires: a rule on triggering, where the card says whose abilities
// Vigilance
// If a permanent entering causes a triggered ability of a permanent you control to trigger, that ability triggers an additional time.
// Permanents entering don't cause abilities of permanents your opponents control to trigger.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { true && (triggering.permanent && Some(asker.controller) == Some(triggering.controller)) && cantrip_mechanics::statics::entering(event).is_some_and(|id| true) }
planned_item!("a-rule-on-triggering-where-the-card-says-whose-abilities", "(kind:TriggerRule,args:(cause:Some(ZoneChange(objects:(kind:All,args:[(kind:Either,args:[(kind:CardType,args:Artifact),(kind:CardType,args:Battle),(kind:CardType,args:Creature),(kind:CardType,args:Enchantment),(kind:CardType,args:Land),(kind:CardType,args:Planeswalker)]),(kind:ControlledBy,args:(kind:OpponentsOf,args:(kind:AbilityController)))]),from:None,to:Battlefield)),abilities:None,rule:DoesntTrigger))");

pub static CARD: CardDef = CardDef { name: "Elesh Norn, Mother of Machines", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Praetor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_triggers: &[ability_1_trigger_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
