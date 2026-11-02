// Fight or Flight
// status: planned
// requires: operation ChangeCombat
// At the beginning of combat on each opponent's turn, separate all creatures that player controls into two piles. Only creatures in the pile of their choice can attack this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
g.end_sentence();
let cards_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && item.subject.player() == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let separator_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let chooser_0 = { let q: &dyn Query = g; item.subject.player().expect("a reference that always names a player") }; let (first_0, second_0) = cantrip_mechanics::choose::separate(g, Some(separator_0), asker.source, cards_0)?; let (pile_0, other_0) = cantrip_mechanics::choose::pile(g, chooser_0, asker.source, first_0, second_0)?;
g.end_sentence();
planned!("operation-change-combat" => cantrip_mechanics::combat::change(g, asker, &{ let q: &dyn Query = g; pile_0.clone() }, CombatChange::OnlyAttackers { added_phase: false })?);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Fight or Flight", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of combat on each opponent's turn, separate all creatures that player controls into two piles. Only creatures in the pile of their choice can attack this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
