// Skyseer's Chariot
// status: ready
// Flying
// As this Vehicle enters, choose a nonland card name.
// Activated abilities of sources with the chosen name cost {2} more to activate.
// Crew 2
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = entering.controller; let Answer::Name(Some(chosen_0)) = g.ask(player_0, RequestKind::ChooseName { source: entering.object, looked_at: Vec::new(), suggestion: None, kind: NameKind { non_types: TypeSet::of(&[CardType::Land]), ..NameKind::ANY } })? else { return Ok(()) };
entering.chosen_card = Some(chosen_0);
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Ability(permanent, ability, _) = costed else { return CostDelta::NONE; }; let def = q.activated_ability(ability); if true && q.state().chosen_card(q.linked_to(asker)).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, permanent, chosen)) { CostDelta::more(2) } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Skyseer's Chariot", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::crew::crew(2, "Crew 2")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, cost_changes: &[ability_2_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
