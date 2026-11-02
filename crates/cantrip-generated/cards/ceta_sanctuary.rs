// Ceta Sanctuary
// status: ready
// At the beginning of your upkeep, if you control a red or green permanent, draw a card, then discard a card. If you control a red permanent and a green permanent, instead draw two cards, then discard a card.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && (q.chars(id).colors.has(q.words(asker.source).color(Color::Red)) || q.chars(id).colors.has(q.words(asker.source).color(Color::Green)))).count() >= 1 }
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && q.chars(id).colors.has(q.words(asker.source).color(Color::Red))).count() >= 1 && q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && q.chars(id).colors.has(q.words(asker.source).color(Color::Green))).count() >= 1) } {
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_1 = Vec::new(); for player in players_1 { cards_1.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_1)?); } let discarded_1 = g.discard_together(&cards_1);
g.end_sentence();
hoisted_2.extend(discarded_1.clone());
hoisted_3.extend({ let q: &dyn Query = g; discarded_1.clone() });
} else {
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
g.end_sentence();
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_5 = Vec::new(); for player in players_5 { cards_5.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_5)?); } let discarded_5 = g.discard_together(&cards_5);
g.end_sentence();
hoisted_6.extend(discarded_5.clone());
hoisted_7.extend({ let q: &dyn Query = g; discarded_5.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Ceta Sanctuary", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, if you control a red or green permanent, draw a card, then discard a card. If you control a red permanent and a green permanent, instead draw two cards, then discard a card.").targets(&[]).condition(ability_0_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
