// Stand or Fall
// status: planned
// requires: object filter InPile
// requires: operation ChangeCombat
// requires: operation ChoosePile
// requires: operation Piles
// requires: per-player ChoosePile
// requires: per-player Piles
// requires: player state PossibleDefender
// At the beginning of combat on your turn, for each defending player, separate all creatures that player controls into two piles and that player chooses one. Only creatures in the chosen piles can block this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|player: PlayerId| planned_value!("player state PossibleDefender", false => q.could_be_attacked(asker.controller, player)))).collect::<Vec<PlayerId>>() };
planned!("per-player-piles" => g.begin_batch(););
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { planned!("operation-piles" => cantrip_mechanics::choose::into_piles(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }, Some({ let q: &dyn Query = g; (2i64).max(0) as u32 }), &[], None)?);
    } 
planned!("per-player-piles" => g.end_batch(););
g.end_sentence();
planned!("per-player-choose-pile" => g.begin_batch(););
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { planned!("operation-choose-pile" => cantrip_mechanics::choose::of_piles(g, { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }, asker.source, None, false, &[], None, None)?);
    } 
planned!("per-player-choose-pile" => g.end_batch(););
g.end_sentence();
planned!("operation-change-combat" => cantrip_mechanics::combat::change(g, asker, CombatChange::OnlyBlockers({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| planned_value!("object filter InPile", false => q.in_pile(asker, id, None))).collect::<Vec<ObjectId>>() }))?);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Stand or Fall", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of combat on your turn, for each defending player, separate all creatures that player controls into two piles and that player chooses one. Only creatures in the chosen piles can block this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
