// Sorcerous Spyglass
// status: planned
// requires: player reference OneOf
// As this artifact enters, look at an opponent's hand, then choose any card name.
// Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let objects_0 = { let q: &dyn Query = g; ((Some(planned_value!("player reference OneOf", asker.controller => q.player_picked_for(asker, Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()))).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; g.look_at(player_0, &objects_0);
let player_1 = entering.controller; let Answer::Name(Some(chosen_1)) = g.ask(player_1, RequestKind::ChooseName { source: entering.object, looked_at: Vec::new(), suggestion: None, kind: NameKind::ANY })? else { return Ok(()) };
entering.chosen_card = Some(chosen_1);
 Ok(()) }
#[allow(unused_variables)] #[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, object: ObjectId, ability: &ActivatedDef) -> bool { let player = q.controller(object); !(true && (q.state().chosen_card(q.linked_to(asker)).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, object, chosen))) && (!(ability.is_mana_ability()))) }

pub static CARD: CardDef = CardDef { name: "Sorcerous Spyglass", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, activation_rules: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
