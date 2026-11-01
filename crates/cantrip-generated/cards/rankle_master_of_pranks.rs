// Rankle, Master of Pranks
// status: ready
// Flying, haste
// Whenever Rankle deals combat damage to a player, choose any number —
// • Each player discards a card.
// • Each player loses 1 life and draws a card.
// • Each player sacrifices a creature of their choice.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_2_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut cards_0 = Vec::new(); for player in players_0 { cards_0.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_0)?); } let discarded_0 = g.discard_together(&cards_0);
g.end_sentence();
 Ok(()) }
fn ability_2_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
 Ok(()) }
fn ability_2_mode_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_3 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_3 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_3 { let eligible: Vec<ObjectId> = objects_3.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_3.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_3.clone(), &[]))? }; for object in taken { picks_3.push((player, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
g.end_sentence();
 Ok(()) }
static ABILITY_2_MODES: ModalDef = modes(&[mode(ability_2_mode_0, "Each player discards a card.").targets(&[]),mode(ability_2_mode_1, "Each player loses 1 life and draws a card.").targets(&[]),mode(ability_2_mode_2, "Each player sacrifices a creature of their choice.").targets(&[])]).choose(0, 3);

pub static CARD: CardDef = CardDef { name: "Rankle, Master of Pranks", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Faerie,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), &ABILITY_2_MODES, "Whenever Rankle deals combat damage to a player, choose any number — Each player discards a card; or Each player loses 1 life and draws a card; or Each player sacrifices a creature of their choice.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
