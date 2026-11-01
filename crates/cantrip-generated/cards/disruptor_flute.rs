// Disruptor Flute
// status: ready
// Flash
// As this artifact enters, choose a card name.
// Spells with the chosen name cost {3} more to cast.
// Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = entering.controller; let Answer::Name(Some(chosen_0)) = g.ask(player_0, RequestKind::ChooseName { source: entering.object, looked_at: Vec::new(), suggestion: None, kind: NameKind::ANY })? else { return Ok(()) };
entering.chosen_card = Some(chosen_0);
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && (q.state().chosen_card(q.linked_to(asker)).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, spell, chosen))) { CostDelta::more(3) } else { CostDelta::NONE } }
#[allow(unused_variables)] #[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, asker: &Asker, object: ObjectId, ability: &ActivatedDef) -> bool { let player = q.controller(object); !(true && (q.state().chosen_card(q.linked_to(asker)).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, object, chosen))) && (!(ability.is_mana_ability()))) }

pub static CARD: CardDef = CardDef { name: "Disruptor Flute", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, activation_rules: &[ability_3_0_rule], cost_changes: &[ability_2_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
