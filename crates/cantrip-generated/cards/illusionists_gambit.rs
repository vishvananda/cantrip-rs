// Illusionist's Gambit
// status: planned
// requires: engine: no effect removes from combat
// requires: engine: no resolving rule on an open set: Prohibit Attack beyond lasting rules
// Cast this spell only during the declare blockers step on an opponent's turn.
// Remove all attacking creatures from combat and untap them. After this phase, there is an additional combat phase. Each of those creatures attacks that combat if able. They can't attack you or planeswalkers you control that combat.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { (q.state().turn.step() == Step::DeclareBlockers && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().any(|bound_0: PlayerId| Some(bound_0) == Some(q.state().turn.active))) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let acted_0: Vec<ObjectId> = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| ((q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() };
planned!("engine-no-effect-removes-from-combat" => for object in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| ((q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() } { g.remove_from_combat(object); });
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (acted_0.clone()).contains(&id)).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_1: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_1 { g.untap(object); } });
g.end_sentence();
g.add_combat_phase();
let happened_3: bool = true;
g.end_sentence();
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-attack-beyond-lasting-rules", CreateContinuousEffect, "affected" = "{\"Objects\":{\"args\":{\"args\":0,\"kind\":\"InResult\"},\"kind\":\"Battlefield\"}}", "changes" = "[{\"args\":{\"RequireCombat\":{\"action\":{\"Attack\":\"Any\"},\"amount\":{\"AtLeast\":{\"args\":1,\"kind\":\"Constant\"}}}},\"kind\":\"Rule\"},{\"args\":{\"Prohibit\":{\"Attack\":{\"PlayersAndPlaneswalkers\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"}}}},\"kind\":\"Rule\"}]", "duration" = "{\"ExtraCombat\":2}", "evaluation" = "\"LiveObjects\"", "source" = { let q: &dyn Query = g; Some(asker.source) });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Illusionist's Gambit", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_only_if: Some(ability_0_if), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
