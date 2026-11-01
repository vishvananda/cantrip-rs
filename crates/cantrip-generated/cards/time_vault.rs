// Time Vault
// status: planned
// requires: replacement of BeginTurn
// This artifact enters tapped.
// This artifact doesn't untap during your untap step.
// If you would begin your turn while this artifact is tapped, you may skip that turn instead. If you do, untap this artifact.
// {T}: Take an extra turn after this one.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
static INSTEAD_1: Question = Question::new("Instead", "", "If you don't, the event goes on as it was.", "Do this instead", "Don't");
#[allow(unused_variables, unused_mut)] fn ability_2_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let objects_2 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_2: Vec<ObjectId> = objects_2.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_2 { g.untap(object); } });
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times = 1u32; g.add_extra_turns(player, times); }
g.end_sentence();
 Ok(()) }
planned_item!("replacement-of-begin-turn" => callbacks: #[allow(unused_variables)] fn ability_2_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::BeginTurn(p) = proposal { let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if Some(asker.controller) == Some(p.player) && Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).tapped) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_2_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::BeginTurn(seen) = &seen { let p = seen; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if let Proposal::BeginTurn(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn ability_2_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::BeginTurn(p) = before { let q: &dyn Query = g; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; let _ = ability_2_0_then_steps(g, asker, subject, data); } }
);
planned_item!("replacement-of-begin-turn" => replacements: replacement(ReplacementKind::BeginTurn, ability_2_0_applies, ability_2_0_apply, "If you would begin your turn while this artifact is tapped, you may skip that turn instead. If you do, untap this artifact.").then(ability_2_0_then).optional(&INSTEAD_1));

pub static CARD: CardDef = CardDef { name: "Time Vault", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::DoesNotUntap]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_3, "{T}: Take an extra turn after this one.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
