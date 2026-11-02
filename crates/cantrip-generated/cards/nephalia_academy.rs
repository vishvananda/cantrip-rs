// Nephalia Academy
// status: planned
// requires: replacement event Discard
// If a spell or ability an opponent controls causes you to discard a card, you may reveal that card and put it on top of your library instead of putting it anywhere else.
// {T}: Add {C}.
#[allow(unused_imports)] use cantrip_api::*;
static INSTEAD_0: Question = Question::new("Instead", " ", "If you don't, the event goes on as it was.", "Do this instead", "Don't");
#[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let owner = q.state().obj(id).owner; let subject = Target::Object(id); let data = [u32::from(owner.0), u32::MAX]; if z.reason == ZoneReason::Discard && Some(asker.controller) == Some(owner) && planned_value!("replacement event Discard", true => q.proposal_cause().is_some_and(|cause| (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(cause)))) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let owner = q.state().obj(id).owner; let subject = Target::Object(id); let data = [u32::from(owner.0), u32::MAX]; if let Proposal::ZoneChange(z) = proposal { z.to = Zone::library(q.state().obj(z.object).owner); z.shuffle = false; } } }
#[allow(unused_variables, unused_mut)] fn ability_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let objects_1 = { let q: &dyn Query = g; subject.object().into_iter().collect::<Vec<ObjectId>>() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
 Ok(()) }
#[allow(unused_variables)] fn ability_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::ZoneChange(z) = before { let q: &dyn Query = g; let id = z.object; let owner = q.state().obj(id).owner; let subject = Target::Object(id); let data = [u32::from(owner.0), u32::MAX]; let _ = ability_0_then_steps(g, asker, subject, data); } }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::C, mana_2_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Nephalia Academy", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(ability_1_mana) }, "{T}: Add {C}.")], statics: &[], replacements: &[replacement(ReplacementKind::ZoneChange, ability_0_applies, ability_0_apply, "If a spell or ability an opponent controls causes you to discard a card, you may reveal that card and put it on top of your library instead of putting it anywhere else.").then(ability_0_then).optional(&INSTEAD_0)], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
