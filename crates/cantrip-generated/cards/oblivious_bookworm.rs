// Oblivious Bookworm
// status: ready
// At the beginning of your end step, you may draw a card. If you do, discard a card unless a permanent entered the battlefield face down under your control this turn or you turned a permanent face up this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if !cant_draw(g, payer_0, 1) && cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)? {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mut done_2: i64 = 0; let mut drawn_2: Vec<ObjectId> = Vec::new(); for player in players_2 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_2); done_2 += g.history().drawn(player) as i64 - before; drawn_2.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
let paid_1 = done_2 > 0;
(paid_1, { let q: &dyn Query = g; (drawn_2).clone() }, Vec::new(), Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
if { let q: &dyn Query = g; !(((q.history().entered().filter(|e| (((e.chars(q).is(CardType::Artifact) || e.chars(q).is(CardType::Battle) || e.chars(q).is(CardType::Creature) || e.chars(q).is(CardType::Enchantment) || e.chars(q).is(CardType::Land) || e.chars(q).is(CardType::Planeswalker)) && q.state().obj(e.permanent).face_down) && Some(asker.controller) == Some(e.controller))).count() as i64) >= (1i64) || (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::TurnedFaceUp { permanent, player } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64))) } {
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_3 = Vec::new(); for player in players_3 { cards_3.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_3)?); } let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
hoisted_4.extend(discarded_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; discarded_3.clone() });
}
g.end_sentence();
hoisted_6.extend(hoisted_4.clone());
hoisted_7.extend({ let q: &dyn Query = g; hoisted_5.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Oblivious Bookworm", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your end step, you may draw a card. If you do, discard a card unless a permanent entered the battlefield face down under your control this turn or you turned a permanent face up this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
